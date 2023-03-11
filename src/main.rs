// Define an Enum PaymentType with variants:
    // DigitalToken
    // Cash

// Define a Seller struct which contains 3 fields:
    // payment_type (PaymentType)
    // price (f32)
    // balance (f32)

// Define a Buyer struct which contains 3 fields:
    // name (String)
    // payment_type (PaymentType)
    // balance (f32)

// Define a BuyerGroup struct which contains:
    // a vector of members (a vector of Buyer struct).

// Implement methods on BuyerGroup:
    // define method add_member
        // which adds a Buyer into members vector 
    
    // define method find_buyer which accepts a PaymentType input
        // that finds returns index of Buyer with matching payment_type, otherwise return -1

    // define buy method which accepts a buyer index and a reference to a seller
        // keeps transferring value of seller's price from buyer to seller, until buyer's balance is insufficient



fn main() {
    // Create 2 buyers with names John, Sally, with payment_type of DigitalToken, Cash, and balance of 100.00 and 100.00 respectively

    // Create an empty BuyerGroup

    // Add 2 buyers (John and Sally) into buyer_group sequentially

    // Create 1 seller with payment_type of Cash, price of 10, balance of 0

    // Call find_buyer method on the buyer group to get index of buyer with Cash payment type

    // Call buy method on the buyer group passing the index of we have obtained right before and the seller
}
