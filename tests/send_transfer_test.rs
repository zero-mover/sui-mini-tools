use stable_test::{AddressKeyPair,send_tranfer_tx,batch_create_account};


fn test_batch_create_account(){
    batch_create_account(100,"./address_data.json","./key_store.keystore");
}


fn test_send_tranfer_tx(){
    
}