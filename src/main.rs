use wasm_bindgen::{
	prelude::wasm_bindgen,
	closure::Closure,
	JsValue,
	JsCast,
};
use web_sys::{
	Document,
	Element,
};
use std::sync::Arc;


struct Folder {
	name: String,
	icon: String,
}

impl Folder {
	fn new(name: &str, icon: &str) -> Folder {
		Folder{name: name.to_string(), icon: icon.to_string()}
	}
}

fn create_folder(folder: Folder) -> Result<Element, JsValue> {
	let div = document().create_element("div")?;
	div.set_class_name("filemanager-file-button");
	siv.set_inner_html(&format!("<img class='filemanager-file-icon' src='./icons/{folder.icon}.svg'><div class='filemanager-file-name'>{folder.name}</div>"))
	//let img = document().create_element("img")?;
	//img.set_class_name("filemanager-file-icon");
	//img.set_attribute("src", &format!("./icons/{}.svg", folder.icon))?;
	//div.append_child(&img)?;
	//let name = document().create_element("div")?;
	//name.set_class_name("filemanager-file-name");
	//name.set_inner_html(&folder.name);
	//div.append_child(&name)?;
	
	Ok(div)
}

struct ListManager {
	managed: Element,
}

impl ListManager {
	fn new(element: Element) -> ListManager {
		ListManager{managed: element}
	}
	fn push(&self, folder: Folder, closure: &Closure<dyn Fn(JsValue)>) -> Result<(), JsValue> {
		let element = create_folder(folder)?;
		element.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref(), ).unwrap();
		self.managed.append_child(&element.into())?;
		Ok(())
	}
	fn remove(&self, index: u32) -> Result<(), JsValue> {
		self.managed.remove_child(&self.managed.children().item(index).ok_or("Child not found")?.into());
		Ok(())
	}
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: String);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

fn document() -> Document {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    document
}
	
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
fn run() -> Result<(), JsValue> {
    let root = document().get_element_by_id("filemanager-files-list").expect("page should have files list root");
    
    log(format!("{:#?}", root));
    
    let list = Arc::new(ListManager::new(root));
    
    let folders = ["application-apk", "application-json", "application-pdf", "application-pgp-encrypted", "application-pgp-signature", "application-postscript", "applications-php", "application-vnd.flatpak", "application-vnd.google-apps.drawing", "folder", "folder-mac", "none"];
    for i in 0..100 {
    	let i2: usize = i;
    	let list2 = Arc::clone(&list);
    	let callback = Closure::new(move |event: JsValue| {
        	list2.remove(i2.try_into().unwrap());
    	});
    	list.push(Folder::new(&format!("{}", i), folders[i%folders.len()]), &callback);
    	callback.forget();
    }
    
    return Ok(());
}
