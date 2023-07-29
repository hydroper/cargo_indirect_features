pub fn f() {
    #[cfg(feature = "c1_some_feature")] {
        println!("called with c1_some_feature");
    }
    #[cfg(not(feature = "c1_some_feature"))] {
        println!("called without c1_some_feature");
    }
}
