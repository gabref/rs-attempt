// OLoKo64

trait Supplier {
    fn get_qrcode(&self) -> Option<String>;
    fn get_status(&self) -> Option<String>;
    fn set_subproduct_api(&mut self, spa: SupplierApi);
    fn from_input_init(&self, config: &str, property: &str) -> Result<SupplierApi, ErrorsSupplier>;
}

trait CreditCardSupplier: Supplier {
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
    fn process(&self, command: CommandEnum) -> Option<String> {
        match command {
            CommandEnum::GetQrCode => self.get_qrcode(),
            CommandEnum::GetStatus => self.get_status(),
        }
    }
}

enum UserMethodEnum {
    CreditCard,
    AnotherMethod,
}

impl TryFrom<&str> for UserMethodEnum {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "CreditCard" => Ok(UserMethodEnum::CreditCard),
            "AnotherMethod" => Ok(UserMethodEnum::AnotherMethod),
            _ => Err(format!("{} is not a valid UserMethodEnum", value)),
        }
    }
}

enum CreditCardSupplierEnum {
    SupplierOne,
    SupplierTwo,
}

impl TryFrom<&str> for CreditCardSupplierEnum {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "SupplierOne" => Ok(CreditCardSupplierEnum::SupplierOne),
            "SupplierTwo" => Ok(CreditCardSupplierEnum::SupplierTwo),
            _ => Err(format!("{} is not a valid CreditCardSupplierEnum", value)),
        }
    }
}
enum CommandEnum {
    GetQrCode,
    GetStatus,
}

impl TryFrom<&str> for CommandEnum {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "get_qrcode" => Ok(CommandEnum::GetQrCode),
            "get_status" => Ok(CommandEnum::GetStatus),
            _ => Err(format!("{} is not a valid CommandEnum", value)),
        }
    }
}

struct SupplierApi {
    supplier_property: Option<String>,
    supplier_configs: Option<String>,
}

impl SupplierApi {
    fn get_supplier_property(&self) -> String {
        self.supplier_property.clone().unwrap_or_default()
    }
    fn set_supplier_configs(&mut self, c: String) {
        self.supplier_configs = Some(c);
    }
}

struct CreditCard {
    selected_product: Option<Box<dyn CreditCardSupplier>>,
}

impl CreditCard {
    fn new() -> Self {
        CreditCard {
            selected_product: None,
        }
    }

    fn set(&mut self, product_name: CreditCardSupplierEnum) {
        match product_name {
            CreditCardSupplierEnum::SupplierOne => {
                self.selected_product = Some(Box::new(SupplierOne {
                    sub_product_api: None,
                }))
            }
            CreditCardSupplierEnum::SupplierTwo => {
                self.selected_product = Some(Box::new(SupplierTwo {
                    sub_product_api: None,
                }))
            }
        }
    }

    fn init_product(&mut self, config: &str, property: &str) {
        if let Some(ref mut selected_product) = self.selected_product {
            selected_product.init(config, property);
        }
    }

    fn process_product(&self, command: CommandEnum) -> Option<String> {
        if let Some(ref selected_product) = self.selected_product {
            selected_product.process(command)
        } else {
            Some(String::from("No product selected"))
        }
    }
}

struct SupplierOne {
    sub_product_api: Option<SupplierApi>,
}

#[derive(Debug)]
enum ErrorsSupplier {
    MissingConfigParameterX,
    MissingConfigParameterY,
}

impl CreditCardSupplier for SupplierOne {}

impl Supplier for SupplierOne {
    fn set_subproduct_api(&mut self, spa: SupplierApi) {
        self.sub_product_api = Some(spa);
    }
    fn get_qrcode(&self) -> Option<String> {
        format!(
            "qrcode:{}",
            self.sub_product_api.as_ref()?.get_supplier_property()
        )
        .into()
    }
    fn get_status(&self) -> Option<String> {
        format!(
            "status:{}",
            self.sub_product_api.as_ref()?.get_supplier_property()
        )
        .into()
    }
    fn from_input_init(&self, config: &str, property: &str) -> Result<SupplierApi, ErrorsSupplier> {
        Ok(SupplierApi {
            supplier_property: Some(property.to_string()),
            supplier_configs: Some(config.to_string()),
        })
    }
}

struct SupplierTwo {
    sub_product_api: Option<SupplierApi>,
}

impl CreditCardSupplier for SupplierTwo {}

impl Supplier for SupplierTwo {
    fn set_subproduct_api(&mut self, spa: SupplierApi) {
        self.sub_product_api = Some(spa);
    }
    fn get_qrcode(&self) -> Option<String> {
        let sub_product_api = self.sub_product_api.as_ref()?;
        format!("qrcode:{}", sub_product_api.get_supplier_property()).into()
    }
    fn get_status(&self) -> Option<String> {
        let sub_product_api = self.sub_product_api.as_ref()?;
        format!("status:{}", sub_product_api.get_supplier_property()).into()
    }
    fn from_input_init(&self, config: &str, property: &str) -> Result<SupplierApi, ErrorsSupplier> {
        Ok(SupplierApi {
            supplier_property: Some(property.to_string()),
            supplier_configs: Some(config.to_string()),
        })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let user_choice: &str = "CreditCard";
    let user_provider_choice_one = "SupplierOne";
    let user_provider_choice_two = "SupplierTwo";
    let user_config_qr = "get_qrcode";
    let user_config_status = "get_status";

    match user_choice.try_into()? {
        UserMethodEnum::CreditCard => {
            let mut product = CreditCard::new();
            product.set(user_provider_choice_one.try_into()?);
            product.init_product("SupplierOne specific configs in a string", "fpr");

            let res = product.process_product(user_config_qr.try_into()?).unwrap();
            println!("Result: {}", res);
            let res = product
                .process_product(user_config_status.try_into()?)
                .unwrap();
            println!("Result: {}", res);

            product.set(user_provider_choice_two.try_into()?);
            product.init_product("SupplierTwo specific configs in a string", "mpr");

            let res = product.process_product(user_config_qr.try_into()?).unwrap();
            println!("Result: {}", res);
            let res = product
                .process_product(user_config_status.try_into()?)
                .unwrap();
            println!("Result: {}", res);
        }
        UserMethodEnum::AnotherMethod => todo!(),
    }

    Ok(())
}

