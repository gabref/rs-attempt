trait CreditCardSupplier {
    fn init(&mut self, config: &str, property: &str) -> String;
    fn process(&self, command: &str) -> String;
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

#[derive(Debug)]
enum ErrorsSupplierOne {
    MissingConfigParameterX,
    MissingConfigParameterY,
}
impl CreditCardSupplier for SupplierOne {
    fn init(&mut self, config: &str, property: &str) -> String {
        let res = self.from_input_init(config, property);
        match res {
            Ok(r) => {
                self.set_subproduct_api(r);
                format!("initialized sub product")
            }
            Err(e) => format!("error happened: {e:?}"),
        }
    }
    fn process(&self, command: &str) -> String {
        match command {
            "get_qrcode" => self.get_qrcode(),
            "get_status" => self.get_status(),
            _ => todo!(),
        }
    }
}

impl SupplierOne {
    fn set_subproduct_api(&mut self, spa: SupplierOneApi) {
        self.sub_product_api = Some(spa);
    }
    fn get_qrcode(&self) -> String {
        format!(
            "qrcode:{}",
            self.sub_product_api
                .as_ref()
                .unwrap()
                .get_supplier_one_property()
        )
    }
    fn get_status(&self) -> String {
        format!(
            "status:{}",
            self.sub_product_api
                .as_ref()
                .unwrap()
                .get_supplier_one_property()
        )
    }
    fn from_input_init(
        &self,
        config: &str,
        property: &str,
    ) -> Result<SupplierOneApi, ErrorsSupplierOne> {
        Ok(SupplierOneApi {
            supplier_one_property: Some(property.to_string()),
            supplier_one_configs: Some(config.to_string()),
        })
    }
}

struct SupplierTwo {
    sub_product_api: Option<SupplierTwoApi>,
}

#[derive(Debug)]
enum ErrorsSupplierTwo {
    MissingConfigParameterA,
    MissingConfigParameterB,
}
impl CreditCardSupplier for SupplierTwo {
    fn init(&mut self, config: &str, property: &str) -> String {
        let res = self.from_input_init(config, property);
        match res {
            Ok(r) => {
                self.set_subproduct_api(r);
                format!("initialized sub product")
            }
            Err(e) => format!("error happened: {e:?}"),
        }
    }
    fn process(&self, command: &str) -> String {
        match command {
            "get_qrcode" => self.get_qrcode(),
            "get_status" => self.get_status(),
            _ => todo!(),
        }
    }
}
impl SupplierTwo {
    fn set_subproduct_api(&mut self, spa: SupplierTwoApi) {
        self.sub_product_api = Some(spa);
    }
    fn get_qrcode(&self) -> String {
        format!(
            "qrcode:{}",
            self.sub_product_api
                .as_ref()
                .unwrap()
                .get_supplier_two_property()
        )
    }
    fn get_status(&self) -> String {
        format!(
            "status:{}",
            self.sub_product_api
                .as_ref()
                .unwrap()
                .get_supplier_two_property()
        )
    }
    fn from_input_init(
        &self,
        config: &str,
        property: &str,
    ) -> Result<SupplierTwoApi, ErrorsSupplierTwo> {
        Ok(SupplierTwoApi {
            supplier_two_property: Some(property.to_string()),
            supplier_two_configs: Some(config.to_string()),
        })
    }
}

fn main() {
    let user_choice = "CreditCard";
    let user_provider_choice = "SupplierOne";
    match user_choice {
        "CreditCard" => {
            let mut product = CreditCard::new();
            product.set(user_provider_choice);
            product.init_product("SupplierOne specific configs in a string", "fpr");

            let res: String = product.process_product("get_qrcode");
            println!("Result: {}", res);
            let res: String = product.process_product("get_status");
            println!("Result: {}", res);

            product.set("SupplierTwo");
            product.init_product("SupplierTwo specific configs in a string", "mpr");

            let res: String = product.process_product("get_qrcode");
            println!("Result: {}", res);
            let res: String = product.process_product("get_status");
            println!("Result: {}", res);
        }
        "AnotherMethod" => todo!(),
        _ => todo!("Method not implemented"),
    }
}

