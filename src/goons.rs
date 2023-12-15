// 420goonsquad420

enum SupplierType {
    SupplierOne,
    SupplierTwo,
}

impl SupplierType {
    fn new (name: &str) -> Self{
        match name {
            "SupplierOne" => Self::SupplierOne,
            "SupplierTwo" => Self::SupplierTwo,
            _ => unimplemented!("Product not implemented"),
        }
    }
}

struct Supplier {
    sub_product_api: SupplierApi,
    name: SupplierType,
}

impl Supplier {
    fn new(product_name: &str, config: &str, property: &str) -> Self {
        Supplier {
            sub_product_api: SupplierApi::from_input_init(config, property),
            name: SupplierType::new(product_name),
        }
    }
    
    fn process(&self, command: &str) -> String {
        match command {
            "get_qrcode" => self.get_qrcode(),
            "get_status" => self.get_status(),
            _ => unimplemented!("Command not implemented"),
        }
    }
    
    fn get_qrcode(&self) -> String {
        format!("qrcode:{}", self.sub_product_api.supplier_property)
    }
    
    fn get_status(&self) -> String {
        format!("status:{}", self.sub_product_api.supplier_property)
    }
}

struct SupplierApi {
    supplier_property: String,
    supplier_configs: String,
}

impl SupplierApi {
    fn from_input_init(config: &str, property: &str) -> Self {
        SupplierApi {
            supplier_property: property.to_string(),
            supplier_configs: config.to_string(),
        }
    }
}

struct CreditCard {
    selected_product: Supplier,
}

impl CreditCard {
    fn new(product_name: &str, config: &str, property: &str) -> Self {
        CreditCard{selected_product: Supplier::new(product_name, config, property)}
    }

    fn process_product(&self, command: &str) -> String {
        self.selected_product.process(command)
    }
}

fn main() {

    let pix1 = CreditCard::new("SupplierOne", "SupplierOne specific configs in a string", "fpr");

    println!("Result: {}", pix1.process_product("get_qrcode"));
    println!("Result: {}", pix1.process_product("get_status"));

    let pix2 = CreditCard::new("SupplierTwo", "SupplierTwo specific configs in a string", "mpr");

    println!("Result: {}", pix2.process_product("get_qrcode"));
    println!("Result: {}", pix2.process_product("get_status"));
}
