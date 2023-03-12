// Define an Enum PaymentType with variants:
    // DigitalToken
    // Cash
#[derive(PartialEq)]
#[derive(Clone)]
enum PaymentType {
    DigitalToken,
    Cash,
}

// Define a Seller struct which contains 3 fields:
    // payment_type (PaymentType)
    // price (f32)
    // balance (f32)

struct Seller {
    payment_type: PaymentType,
    price: f32,
    balance: f32,
}

// Define a Buyer struct which contains 3 fields:
    // name (String)
    // payment_type (PaymentType)
    // balance (f32)

#[derive(Clone)]
struct Buyer {
    name: String,
    payment_type: PaymentType,
    balance: f32,
}

// Define a BuyerGroup struct which contains:
    // a vector of members (a vector of Buyer struct).
struct BuyerGroup {
    members: Vec<Buyer>,
}

// Implement methods on BuyerGroup:
    // define method add_member
        // which adds a Buyer into members vector 
    
    // define method find_buyer which accepts a PaymentType input
        // that finds returns index of Buyer with matching payment_type, otherwise return -1

    // define buy method which accepts a buyer index and a reference to a seller
        // keeps transferring value of seller's price from buyer to seller, until buyer's balance is insufficient

impl BuyerGroup {
    fn add_member(&mut self, buyer: &Buyer) {
        self.members.push(buyer.clone());
    }

    fn find_buyer(&self, buyer_payment_type: PaymentType) -> i32 {
        for (i, member) in self.members.iter().enumerate() {
            if member.payment_type == buyer_payment_type {
                return i as i32;
            }
        }

        -1
    }

    fn buy(&mut self, buyer_index: usize, seller: &mut Seller) {
        if buyer_index >= self.members.len() {
            return;
        }

        let buyer = &mut self.members[buyer_index];

        // Transfer amount from "buyer" -> "seller"
        while buyer.balance >= seller.price {
            buyer.balance -= seller.price;
            seller.balance += seller.price;
        }
    }
}



fn main() {
    // Create 2 buyers with names John, Sally, with payment_type of DigitalToken, Cash, and balance of 100.00 and 100.00 respectively
    let john = Buyer {
        name: "John".to_owned(),
        payment_type: PaymentType::DigitalToken,
        balance: 100.0,
    };

    let sally = Buyer {
        name: "Sally".to_owned(),
        payment_type: PaymentType::Cash,
        balance: 100.0
    };

    // Create an empty BuyerGroup
    let mut buyer_group = BuyerGroup {
        members: Vec::new()
    };

    // Add 2 buyers (John and Sally) into buyer_group sequentially
    buyer_group.add_member(&john);
    buyer_group.add_member(&sally);

    assert_eq!(buyer_group.members.len(), 2, "Buyer group does not have 2 buyers");

    // Create 1 seller with payment_type of Cash, price of 10, balance of 0
    let mut james_seller = Seller {
        payment_type: PaymentType::Cash,
        price: 10.0,
        balance: 0.0,
    };

    // Call find_buyer method on the buyer group to get index of buyer with Cash payment type
    let buyer_index: i32 = buyer_group.find_buyer(PaymentType::Cash);
    assert_eq!(buyer_index, 1, "Buyer index is supposed to be Sally at index 1");

    // Call buy method on the buyer group passing the index of we have obtained right before and the seller
    buyer_group.buy(buyer_index as usize, &mut james_seller);

    let buyer_group_members = buyer_group.members;
    let buyer_payment_cash = &buyer_group_members[buyer_index.abs() as usize];
    
    // Buyer balance should be left with the remainder of the "balance" / "price"
    assert_eq!(buyer_payment_cash.balance, sally.balance % james_seller.price);
    // Seller balance should be increased by difference in buyer's balance
    assert_eq!(james_seller.balance, sally.balance - buyer_payment_cash.balance)

}
