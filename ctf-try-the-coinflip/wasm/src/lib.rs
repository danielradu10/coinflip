// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Upgrade:                              1
// Endpoints:                            3
// Async Callback (empty):               1
// Total number of exported functions:   6

#![no_std]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    ctf_try_the_coinflip
    (
        init => init
        upgrade => upgrade
        call_the_coinflip => call_the_coinflip
        bumps => bumps
        donateBumps => donate_bumps
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
