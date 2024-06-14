#[derive(Debug)]
pub struct RcBitmap {
    pub id: String,
    pub file: String,
}

pub struct RcBitmapList {
    pub bitmaps: Vec<RcBitmap>,
}

impl RcBitmapList {
    pub fn new() -> Self {
        RcBitmapList { bitmaps: vec![] }
    }

    pub fn add(&mut self, bitmap: RcBitmap) {
        self.bitmaps.push(bitmap);
    }

    pub fn output(&self) {
        println!("void qtMfcInitBitmapResources() {{");
        println!("    qtMfcBitmapResources.clear();");
        for bitmap in &self.bitmaps {
            println!("    // {} BITMAP {}", bitmap.id, bitmap.file);
            println!(
                "    qtMfcBitmapResources.insert({}, new CBitmap({}));",
                bitmap.id, bitmap.file
            );
        }
        println!("}}");
    }
}

#[derive(Debug)]
pub struct RcIcon {
    pub id: String,
    pub file: String,
}

pub struct RcIconList {
    pub icons: Vec<RcIcon>,
}

impl RcIconList {
    pub fn new() -> Self {
        RcIconList { icons: vec![] }
    }

    pub fn add(&mut self, icon: RcIcon) {
        self.icons.push(icon);
    }

    pub fn output(&self) {
        println!("void qtInitIconResources() {{");
        println!("    qtIconNames.clear();");
        println!("    qtIconResources.clear();");
        println!("    // Icon with lowest ID value placed first to ensure application icon remains consistent on all systems.");
        for icon in &self.icons {
            println!("    // {} ICON {}", icon.id, icon.file);
            println!("    qtIconNames.insert({}, {});", icon.id, icon.file);
        }
        println!("}}");
    }
}

#[derive(Debug)]
pub struct RcStringTableItem {
    pub id: String,
    pub text: String,
}

pub struct RcStringTable {
    pub table: Vec<RcStringTableItem>,
}

impl RcStringTable {
    pub fn new(table: Vec<RcStringTableItem>) -> Self {
        RcStringTable { table }
    }

    pub fn output(&self) {
        for item in &self.table {
            println!(
                "    qtMfcStringResources.insert({}, \"{}\");",
                item.id, item.text
            );
        }
    }
}

pub struct RcStringTableList {
    pub string_tables: Vec<RcStringTable>,
}

impl RcStringTableList {
    pub fn new() -> Self {
        RcStringTableList {
            string_tables: vec![],
        }
    }

    pub fn add(&mut self, string_table: RcStringTable) {
        self.string_tables.push(string_table);
    }

    pub fn output(&self) {
        println!("void qtMfcInitStringResources() {{");
        println!("    qtMfcStringResources.clear();");
        for string_table in &self.string_tables {
            println!("    // STRINGTABLE");
            println!("    // BEGIN");
            string_table.output();
            println!("    // END");
        }
        println!(
            "    // AFX resources
            qtMfcStringResources.insert(AFX_IDS_ALLFILTER, \"All files|\");
            qtMfcStringResources.insert(AFX_IDS_OPENFILE, \"Open\");
            qtMfcStringResources.insert(AFX_IDS_SAVEFILE, \"Save As\");
            qtMfcStringResources.insert(AFX_IDS_SAVEFILECOPY, \"Save As\");
            qtMfcStringResources.insert(AFX_IDS_UNTITLED, \"Untitled\");
            qtMfcStringResources.insert(AFX_IDP_ASK_TO_SAVE, \"Save changes to %s?\");
            qtMfcStringResources.insert(AFX_IDP_FAILED_TO_CREATE_DOC, \"Failed to create empty document.\");
        ");
        println!("}}");
    }
}

// Define other structures similarly: RcToolbar, RcAcceleratorTable, RcMenu, RcDialogEx
// Implement corresponding list structures: RcToolbarList, RcAcceleratorTableList, RcMenuList, RcDialogExList

// RcToolbar structure
#[derive(Debug)]
pub struct RcToolbar {
    pub id: String,
    pub numbers: Vec<i32>,
    pub table: Vec<String>, // Assuming a simplified type for demonstration
}

pub struct RcToolbarList {
    pub toolbars: Vec<RcToolbar>,
}

impl RcToolbarList {
    pub fn new() -> Self {
        RcToolbarList { toolbars: vec![] }
    }

    pub fn add(&mut self, toolbar: RcToolbar) {
        self.toolbars.push(toolbar);
    }

    pub fn output(&self) {
        for toolbar in &self.toolbars {
            println!(
                "void qtMfcInitToolBarResource_{}(UINT dlgID, CToolBar* parent) {{",
                toolbar.id
            );
            println!("    // Toolbar logic here...");
        }
        println!("void qtMfcInitToolBarResource(UINT dlgID, CToolBar* parent) {{");
        println!("    switch (dlgID) {{");
        for toolbar in &self.toolbars {
            println!("        case {}:", toolbar.id);
            println!(
                "            qtMfcInitToolBarResource_{}(dlgID, parent);",
                toolbar.id
            );
            println!("            break;");
        }
        println!("    }}");
        println!("}}");
    }
}

// RcAcceleratorTable structure
#[derive(Debug)]
pub struct RcAcceleratorTable {
    pub id: String,
    pub table: Vec<String>, // Assuming a simplified type for demonstration
}

pub struct RcAcceleratorTableList {
    pub accelerator_tables: Vec<RcAcceleratorTable>,
}

impl RcAcceleratorTableList {
    pub fn new() -> Self {
        RcAcceleratorTableList {
            accelerator_tables: vec![],
        }
    }

    pub fn add(&mut self, accelerator_table: RcAcceleratorTable) {
        self.accelerator_tables.push(accelerator_table);
    }

    pub fn output(&self) {
        for accelerator_table in &self.accelerator_tables {
            println!("// {} ACCELERATORS", accelerator_table.id);
            println!("ACCEL ACCEL_{}[] = {{", accelerator_table.id);
            println!("    // Accelerator table logic here...");
            println!("    {{ 0, 0, 0 }},");
            println!("}};");
        }
        println!("ACCEL* qtMfcAcceleratorResource(UINT id) {{");
        println!("    switch (id) {{");
        for accelerator_table in &self.accelerator_tables {
            println!("        case {}:", accelerator_table.id);
            println!("            return ACCEL_{};", accelerator_table.id);
            println!("            break;");
        }
        println!("    }}");
        println!("    return NULL;");
        println!("}}");
    }
}

// RcMenu structure
#[derive(Debug)]
pub struct RcMenu {
    pub id: String,
    pub table: Vec<String>, // Assuming a simplified type for demonstration
}

pub struct RcMenuList {
    pub menus: Vec<RcMenu>,
}

impl RcMenuList {
    pub fn new() -> Self {
        RcMenuList { menus: vec![] }
    }

    pub fn add(&mut self, menu: RcMenu) {
        self.menus.push(menu);
    }

    pub fn output(&self) {
        for menu in &self.menus {
            println!("void qtMfcInitMenuResource_{}(CMenu* parent) {{", menu.id);
            println!("    // Menu logic here...");
        }
        println!("void qtMfcInitMenuResource(UINT menuID, CMenu* parent) {{");
        println!("    switch (menuID) {{");
        for menu in &self.menus {
            println!("        case {}:", menu.id);
            println!("            qtMfcInitMenuResource_{}(parent);", menu.id);
            println!("            break;");
        }
        println!("    }}");
        println!("    // Fixup shortcuts");
        println!("    int menu = 0;");
        println!("    CMenu* subMenu = parent->GetSubMenu(menu);");
        println!("    while (subMenu) {{");
        println!("        foreach (QAction* action, subMenu->toQMenu()->actions()) {{");
        println!("            if (action->text().contains(\"\\t\")) {{");
        println!("                action->setShortcut(QKeySequence(action->text().split(\"\\t\").at(1)));");
        println!("            }}");
        println!("        }}");
        println!("        menu++;");
        println!("        subMenu = parent->GetSubMenu(menu);");
        println!("    }}");
        println!("}}");
    }
}

// RcDialogEx structure
#[derive(Debug)]
pub struct RcDialogEx {
    pub id: String,
    pub items: Vec<String>,   // Assuming a simplified type for demonstration
    pub options: Vec<String>, // Assuming a simplified type for demonstration
    pub numbers: Vec<i32>,
    pub caption: String,
    pub style: String,
    pub font: String,
}

pub struct RcDialogExList {
    pub dialogs: Vec<RcDialogEx>,
}

impl RcDialogExList {
    pub fn new() -> Self {
        RcDialogExList { dialogs: vec![] }
    }

    pub fn add(&mut self, dialog: RcDialogEx) {
        self.dialogs.push(dialog);
    }

    pub fn output(&self) {
        for dialog in &self.dialogs {
            println!(
                "void qtMfcInitDialogResource_{}(CDialog* parent) {{",
                dialog.id
            );
            println!("    // Dialog logic here...");
        }
        println!("void qtMfcInitDialogResource(UINT dlgID, CDialog* parent) {{");
        println!("    switch (dlgID) {{");
        for dialog in &self.dialogs {
            println!("        case {}:", dialog.id);
            println!("            qtMfcInitDialogResource_{}(parent);", dialog.id);
            println!("            break;");
        }
        println!("        case 0:");
        println!("            // Allow blank dialogs.");
        println!("            break;");
        println!("        default:");
        println!("            qFatal(\"dialog resource not implemented...\");");
        println!("    }}");
        println!("}}");
    }
}
