// Define a trait for the common functionalities of Pix products
trait CreditCardSupplier {
    fn init(&mut self, config: &str, property: &str) -> String {
        let f = self.from_input_init(config, property);
        self.set_subproduct_api(f);
        format!("initialized sub product")
    }
    fn process(&self, command: &str) -> String {
        match command {
            "get_qrcode" => self.get_qrcode(),
            "get_status" => self.get_status(),
            _ => todo!(),
        }
    }
    fn get_qrcode(&self) -> String;
    fn get_status(&self) -> String;
    fn set_subproduct_api(&mut self, spa: Box<dyn SubProductApi>);
    fn from_input_init(&self, config: &str, property: &str) -> Box<dyn SubProductApi>;
}

// Define the trait for the sub-product API
trait SubProductApi {}

// Implement the trait for FastCash API
struct SupplierOneApi {
    fastcash_property: Option<String>,
    fastcash_configs: Option<String>,
}
impl SupplierOneApi {
    fn get_supplier_one_property(&self) -> String {
        self.fastcash_property.clone().unwrap_or_default()
    }
    fn set_supplier_one_configs(&mut self, c: String) {
        self.fastcash_configs = Some(c);
    }
}

impl SubProductApi for SupplierOneApi {}

// Implement the trait for Matera API
struct SupplierTwoApi {
    matera_property: Option<String>,
    matera_configs: Option<String>,
}
impl SupplierTwoApi {
    fn get_supplier_two_property(&self) -> String {
        self.matera_property.clone().unwrap_or_default()
    }
    fn set_supplier_two_configs(&mut self, c: String) {
        self.matera_configs = Some(c);
    }
}

impl SubProductApi for SupplierTwoApi {}

// Define the Pix struct with a generic type parameter
struct CreditCard {
    selected_product: Option<Box<dyn CreditCardSupplier>>,
}

// Implement methods for Pix
impl CreditCard {
    fn new() -> Self {
        CreditCard {
            selected_product: None,
        }
    }

    fn set(&mut self, product_name: &str) {
        // Initialize the selected product based on the product name
        match product_name {
            "SupplierOne" => {
                self.selected_product = Some(Box::new(SupplierOne {
                    sub_product_api: None,
                }))
            }
            "SupplierTwo" => {
                self.selected_product = Some(Box::new(SupplierTwo {
                    sub_product_api: None,
                }))
            }
            _ => unimplemented!("Product not implemented"),
        }
    }

    fn init_product(&mut self, config: &str, property: &str) {
        // Call the init method on the selected product
        if let Some(ref mut selected_product) = self.selected_product {
            selected_product.init(config, property);
        }
    }

    fn process_product(&self, command: &str) -> String {
        // Call the process method on the selected product
        if let Some(ref selected_product) = self.selected_product {
            selected_product.process(command)
        } else {
            String::from("No product selected")
        }
    }
}

// Implement PixProduct for FastCash
struct SupplierOne {
    sub_product_api: Option<SupplierOneApi>,
}

impl CreditCardSupplier for SupplierOne {
    fn get_qrcode(&self) -> String {
        format!(
            "qrcode:{}",
            self.sub_product_api.as_ref().unwrap().get_supplier_one_property()
        )
    }
    fn get_status(&self) -> String {
        format!(
            "status:{}",
            self.sub_product_api.as_ref().unwrap().get_supplier_one_property()
        )
    }
    fn set_subproduct_api(&mut self, spa: Box<dyn SubProductApi>) {
        self.sub_product_api = Some(spa);
    }
    fn from_input_init(&self, config: &str, property: &str) -> Box<dyn SubProductApi> {
        Box::new(SupplierOneApi {
            fastcash_property: Some(property.to_string()),
            fastcash_configs: Some(config.to_string()),
        })
    }
}

// Implement PixProduct for Matera
struct SupplierTwo {
    sub_product_api: Option<SupplierTwoApi>,
}

impl CreditCardSupplier for SupplierTwo {
    fn get_qrcode(&self) -> String {
        format!(
            "qrcode:{}",
            self.sub_product_api.as_ref().unwrap().get_supplier_two_property()
        )
    }
    fn get_status(&self) -> String {
        format!(
            "status:{}",
            self.sub_product_api.as_ref().unwrap().get_supplier_two_property()
        )
    }
    fn set_subproduct_api(&mut self, spa: Box<dyn SubProductApi>) {
        self.sub_product_api = Some(spa);
    }
    fn from_input_init(&self, config: &str, property: &str) -> Box<dyn SubProductApi> {
        Box::new(SupplierTwoApi {
            matera_property: Some(property.to_string()),
            matera_configs: Some(config.to_string()),
        })
    }
}

fn main() {
    // Example usage

    let mut pix = CreditCard::new();
    pix.set("SupplierOne");
    pix.init_product("SupplierOne specific configs in a string", "fpr");

    let res: String = pix.process_product("get_qrcode");
    println!("Result: {}", res);
    let res: String = pix.process_product("get_status");
    println!("Result: {}", res);

    pix.set("SupplierTwo");
    pix.init_product("SupplierTwo specific configs in a string", "mpr");

    let res: String = pix.process_product("get_qrcode");
    println!("Result: {}", res);
    let res: String = pix.process_product("get_status");
    println!("Result: {}", res);
}

