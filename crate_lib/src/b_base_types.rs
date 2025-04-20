pub mod base_types{
    pub fn use_option()->Option<String>{
        None //返回值
    }

    //主要返回<T,E>T:success和E:error
    pub fn use_result()->Result<(),u8>{
        Ok(())//不以分号结束，默认为返回值
    }
}