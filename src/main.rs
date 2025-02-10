fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    #[test]
    fn test_smart_pointer() {
        let get_value = |v: i32| {
            println!("{}", v);
        };
        let get_value_reference = |v: &i32| {
            println!("{}", v);
        };

        //TEST SMART POINTER
        let x = Box::new(14);

        println!("{}", x);
        get_value(*x);
        get_value_reference(&x);
    }

    #[test]
    fn test_smart_pointers() {
        // Mengijinkan kita membuat data di heap namun pointernya disimpan di stack
        let a = Box::new(10);
        let b = &a; //borrow ke a

        assert_eq!(*a, **b) // Dereference kedua pointer untuk membandingkan nilai asli
    }

    #[test]
    fn test_usage_smart_pointer() {
        #[derive(Debug)]
        enum Product {
            Of(String, Box<Product>),
            End,
        }

        let car = Product::Of(
            "CAR".to_string(),
            Box::new(Product::Of("PORCHE".to_string(), Box::new(Product::End))),
        );

        println!("{:?}", car);
    }

    #[test]
    fn foobar() {
        let mut count = 0;
        loop {
            println!("wehhh ke {}", count);

            if count == 10 {
                break
            }
            count+=1;
        }
    }

    #[test]
    fn test_aja() {
        let txt = String::from("EheheEH");
        let txt2 = "ehehe"+ &txt;
    }


}