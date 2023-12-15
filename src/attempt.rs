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

trait SubProductApi {}

struct SupplierOneApi {
    supplier_one_property: Option<String>,
    supplier_one_configs: Option<String>,
}
impl SupplierOneApi {
    fn get_supplier_one_property(&self) -> String {
        self.supplier_one_property.clone().unwrap_or_default()
    }
    fn set_supplier_one_configs(&mut self, c: String) {
        self.supplier_one_configs = Some(c);
    }
}

impl SubProductApi for SupplierOneApi {}

struct SupplierTwoApi {
    supplier_two_property: Option<String>,
    supplier_two_configs: Option<String>,
}
impl SupplierTwoApi {
    fn get_supplier_two_property(&self) -> String {
        self.supplier_two_property.clone().unwrap_or_default()
    }
    fn set_supplier_two_configs(&mut self, c: String) {
        self.supplier_two_configs = Some(c);
    }
}

impl SubProductApi for SupplierTwoApi {}

struct CreditCard {
    selected_product: Option<Box<dyn CreditCardSupplier>>,
}

impl CreditCard {
    fn new() -> Self {
        CreditCard {
            selected_product: None,
        }
    }

    fn set(&mut self, product_name: &str) {
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
        if let Some(ref mut selected_product) = self.selected_product {
            selected_product.init(config, property);
        }
    }

    fn process_product(&self, command: &str) -> String {
        if let Some(ref selected_product) = self.selected_product {
            selected_product.process(command)
        } else {
            String::from("No product selected")
        }
    }
}

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
            supplier_one_property: Some(property.to_string()),
            supplier_one_configs: Some(config.to_string()),
        })
    }
}

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
            supplier_two_property: Some(property.to_string()),
            supplier_two_configs: Some(config.to_string()),
        })
    }
}

fn main() {

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

