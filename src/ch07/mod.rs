pub mod d;

pub mod a {
    pub fn afn1() {
        println!("hello afn1");
        // access private sibling fn
        afn2();
        // access private sibling mod
        super::b::bfn1();
        // access private fn in parent's mod
        super::cfn1();
        // access module define in another file
        super::d::dfn1();
    }

    fn afn2() {
        println!("hello afn2");
    }
}

mod b {
    pub fn bfn1() {
        println!("hello bfn1");
    }
}

fn cfn1() {
    println!("hello cfn1");
}