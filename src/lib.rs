use std::io;
#[derive(Debug)]
struct Celsius(pub f64);
#[derive(Debug)]
struct Fahrenheit(pub f64);

impl Celsius {
    fn new(n: f64) -> Self {
        Self(n)
    }
    fn to_fa(&self) -> Fahrenheit {
        let f = (self.0 * (9_f64 / 5_f64)) + 32_f64;
        Fahrenheit(f)
    }
}
impl Fahrenheit {
    fn new(n: f64) -> Self {
        Self(n)
    }
    fn to_cel(&self) -> Celsius {
        let c = (self.0 - 32_f64) * (5_f64 / 9_f64);
        Celsius(c)
    }
}

pub fn init() -> io::Result<()> {
    loop {
        let mut convert_choice = String::new();
        let mut value_to_convert = String::new();
        println!("Pls choose the one most appropriate: \n1. From Celcius to Farenheit.\n2. From Farenheit to Celsius.\n3. Quit.");
        io::stdin().read_line(&mut convert_choice)?;

        let choice = convert_choice.trim().parse::<i32>().unwrap();
        if choice == 1 {
            println!("Pls provide the value to convert (Intergers of Floating points):");
            io::stdin().read_line(&mut value_to_convert)?;
            let Ok(value) = value_to_convert.trim().parse::<f64>() else {
                println!("Invalid data suplied");
                break;
            };

            let c = Celsius::new(value as f64);
            let to_fa = c.to_fa();
            let Fahrenheit(x) = to_fa;
            println!(
                "\n{} Celcius is equivalent to: {:.2} Fahrenheit\n",
                value, x
            );
        } else if choice == 2 {
            println!("Pls provide the value to convert (Intergers of Floating points):");
            io::stdin().read_line(&mut value_to_convert)?;
            let Ok(value) = value_to_convert.trim().parse::<f64>() else {
                println!("Invalid data suplied");
                break;
            };
            let c = Fahrenheit::new(value as f64);
            let to_cel = c.to_cel();
            let Celsius(x) = to_cel;
            println!(
                "\n{} Fahrenheit is equivalent to: {:.2} Celsius\n",
                value, x
            );
        } else if choice == 3 {
            println!("See ya!");
            break;
        } else {
            println!("Select 1 or 2");
        }
    }
    Ok(())
}
