pub fn f() {
    #[cfg(feature = "c1_some_feature")] {
        println!("called c3::f() with c1_some_feature");
    }
    #[cfg(not(feature = "c1_some_feature"))] {
        println!("called c3::f() without c1_some_feature");
    }
}
