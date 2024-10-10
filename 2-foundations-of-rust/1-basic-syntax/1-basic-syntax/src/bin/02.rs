fn main() {
    if bigger(10, 20) == 10 {
        println!("10 is bigger than 20");
    } else {
        println!("10 still isn't bigger than 20");
    }
}

fn bigger(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
      b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_biggers() {
        assert_eq!(bigger(20, 10), 20); 
        assert_eq!(bigger(10, 20), 20);  
    }
}
