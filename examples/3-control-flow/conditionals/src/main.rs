fn main() {
    let proceed = true;
    if proceed {
        println!("Proceeding");
    } else {
        println!("Not proceeding");
    }

    let height = 190;
    if height > 180 && proceed == true {
        println!("Tall");
    } else if height > 170 && proceed == true{
        println!("Average");
    } else {
        if proceed == false {
            println!("Unable to proceeding");
        } else {
        println!("Short");
        }
    }

    let age = 9;

    if age > 18 {
        println!("Adult");
    } else if age > 12 {
        println!("Teenager");
    } else {
        println!("Child");
    }

}
