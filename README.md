# blueshift_anchor_escrow
This is the escrow exercise(the second one) by blueshift after completing introductory theory of anchor. 

An escrow is a powerful financial tool that enables secure token swaps between two parties.

Think of it as a digital safe deposit box where one user can lock up Token A, waiting for another user to deposit Token B before the swap is completed.

This creates a trustless environment where neither party needs to worry about the other backing out of the deal.

Going to implement this concept through three simple but powerful instructions:

- Make: The maker (first user) defines the trade terms and deposits the agreed amount of Token A into a secure vault. This is like putting your item in the safe deposit box and setting the terms of the exchange.
- Take: The taker (second user) accepts the offer by transferring the promised amount of Token B to the maker, and in return, receives the locked Token A. This is the moment when both parties complete their side of the deal.
- Refund: If the maker changes their mind or no suitable taker is found, they can cancel the offer and retrieve their Token A. This is like getting your item back from the safe deposit box if the deal falls through.
