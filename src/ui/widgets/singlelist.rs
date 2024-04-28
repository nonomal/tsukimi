use super::item::ItemPage;
use super::movie::MoviePage;
use super::tu_list_item::TuListItem;
use super::window::Window;
use crate::client::{network::*, structs::*};
use crate::ui::provider::tu_item::TuItem;
use crate::utils::{get_data_with_cache, spawn, spawn_tokio};
use adw::prelude::NavigationPageExt;
use glib::Object;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};
mod imp {

    use std::cell::{OnceCell, RefCell};

    use glib::subclass::InitializingObject;
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;
    use gtk::{glib, CompositeTemplate};

    use crate::utils::spawn_g_timeout;

    // Object holding the state
    #[derive(CompositeTemplate, Default, glib::Properties)]
    #[template(resource = "/moe/tsukimi/singlelist.ui")]
    #[properties(wrapper_type = super::SingleListPage)]
    pub struct SingleListPage {
        #[property(get, set, construct_only)]
        pub id: OnceCell<String>,
        #[property(get, set, nullable)]
        pub parentid: RefCell<Option<String>>,
        #[property(get, set, construct_only)]
        pub collectiontype: OnceCell<String>,
        #[property(get, set, construct_only)]
        pub listtype: OnceCell<String>,
        #[template_child]
        pub listgrid: TemplateChild<gtk::GridView>,
        #[template_child]
        pub spinner: TemplateChild<gtk::Spinner>,
        #[template_child]
        pub listrevealer: TemplateChild<gtk::Revealer>,
        #[template_child]
        pub count: TemplateChild<gtk::Label>,
        #[template_child]
        pub listscrolled: TemplateChild<gtk::ScrolledWindow>,
        #[template_child]
        pub postmenu: TemplateChild<gtk::MenuButton>,
        #[template_child]
        pub dropdown: TemplateChild<gtk::DropDown>,
        #[template_child]
        pub adbutton: TemplateChild<gtk::Box>,
        pub selection: gtk::SingleSelection,
        pub popovermenu: RefCell<Option<gtk::PopoverMenu>>,
    }

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for SingleListPage {
        // `NAME` needs to match `class` attribute of template
        const NAME: &'static str = "SingleListPage";
        type Type = super::SingleListPage;
        type ParentType = adw::NavigationPage;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    // Trait shared by all GObjects
    #[glib::derived_properties]
    impl ObjectImpl for SingleListPage {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            spawn_g_timeout(glib::clone!(@weak obj => async move {
                obj.handle_type().await;
                obj.set_factory("Descending").await;
            }));
        }
    }

    // Trait shared by all widgets
    impl WidgetImpl for SingleListPage {}

    // Trait shared by all windows
    impl WindowImpl for SingleListPage {}

    // Trait shared by all application windows
    impl ApplicationWindowImpl for SingleListPage {}

    impl adw::subclass::navigation_page::NavigationPageImpl for SingleListPage {}
}

glib::wrapper! {
    pub struct SingleListPage(ObjectSubclass<imp::SingleListPage>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget ,adw::NavigationPage,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

#[gtk::template_callbacks]
impl SingleListPage {
    pub fn new(
        id: String,
        collection_type: String,
        listtype: &str,
        parentid: Option<String>,
    ) -> Self {
        Object::builder()
            .property("id", id)
            .property("collectiontype", collection_type)
            .property("listtype", listtype)
            .property("parentid", parentid)
            .build()
    }

    #[template_callback]
    async fn sort_order_ascending_cb(&self,_btn: &gtk::ToggleButton) {
        self.sortorder("Ascending").await;
    }

    #[template_callback]
    async fn sort_order_descending_cb(&self,_btn: &gtk::ToggleButton) {
        self.sortorder("Descending").await;
    }

    async fn handle_type(&self) {
        let imp = self.imp();
        let listtype = imp.listtype.get().unwrap();
        match listtype.as_str() {
            "all" => {}
            "resume" => {
                imp.postmenu.set_visible(false);
                imp.dropdown.set_visible(false);
                imp.adbutton.set_visible(false);
            }
            "boxset" => {
                imp.postmenu.set_visible(false);
            }
            "tags" => {
                imp.postmenu.set_visible(false);
                imp.dropdown.set_visible(false);
                imp.adbutton.set_visible(false);
            }
            "genres" => {
                imp.postmenu.set_visible(false);
                imp.dropdown.set_visible(false);
                imp.adbutton.set_visible(false);
            }
            "liked" => {
                imp.postmenu.set_visible(false);
            }
            _ => {
                imp.postmenu.set_visible(false);
            }
        }
    }

    async fn set_factory(&self, order: &str) {
        let order = order.to_owned();
        let update_order = order.clone();
        let imp = self.imp();
        let spinner = imp.spinner.get();
        let listrevealer = imp.listrevealer.get();
        let count = imp.count.get();
        let id = imp.id.get().expect("id not set").clone();
        let include_item_types = self.get_include_item_types().to_owned();
        let listtype = imp.listtype.get().unwrap().clone();
        spinner.set_visible(true);
        let parentid = imp.parentid.borrow().clone();
        let list_results = get_data_with_cache(
            id.to_string(),
            &format!("{}{}", listtype.clone(), include_item_types),
            async move {
                if let Some(parentid) = parentid {
                    get_inlist(parentid.to_string(), 0.to_string(), &listtype, &id, &order).await
                } else {
                    get_list(
                        id.to_string(),
                        0.to_string(),
                        &include_item_types,
                        &listtype,
                        &order,
                    )
                    .await
                }
            },
        )
        .await
        .unwrap();
        let store = gio::ListStore::new::<glib::BoxedAnyObject>();
        spawn(glib::clone!(@weak store=> async move {
                for result in list_results.items {
                    let object = glib::BoxedAnyObject::new(result);
                    store.append(&object);
                }
                spinner.set_visible(false);
                count.set_text(&format!("{} Items",list_results.total_record_count));
                listrevealer.set_reveal_child(true);
        }));
        imp.selection.set_model(Some(&store));
        let factory = gtk::SignalListItemFactory::new();
        let listtype = imp.listtype.get().unwrap().clone();
        factory.connect_bind(move |_, item| {
            let list_item = item
                .downcast_ref::<gtk::ListItem>()
                .expect("Needs to be ListItem");
            let entry = item
                .downcast_ref::<gtk::ListItem>()
                .expect("Needs to be ListItem")
                .item()
                .and_downcast::<glib::BoxedAnyObject>()
                .expect("Needs to be BoxedAnyObject");
            let latest: std::cell::Ref<Latest> = entry.borrow();
            if list_item.child().is_none() {
                match latest.latest_type.as_str() {
                    "Movie" => {
                        let tu_item: TuItem = glib::object::Object::new();
                        tu_item.set_id(latest.id.clone());
                        tu_item.set_name(latest.name.clone());
                        tu_item.set_production_year(latest.production_year.unwrap_or_else(|| 0));
                        if let Some(userdata) = &latest.user_data {
                            tu_item.set_played(userdata.played);
                        }
                        let list_child = TuListItem::new(tu_item, "Movie", listtype == "resume");
                        list_item.set_child(Some(&list_child));
                    }
                    "Series" => {
                        let tu_item: TuItem = glib::object::Object::new();
                        tu_item.set_id(latest.id.clone());
                        tu_item.set_name(latest.name.clone());
                        tu_item.set_production_year(latest.production_year.unwrap());
                        if let Some(userdata) = &latest.user_data {
                            tu_item.set_played(userdata.played);
                            tu_item.set_unplayed_item_count(userdata.unplayed_item_count.unwrap());
                        }
                        let list_child = TuListItem::new(tu_item, "Series", listtype == "resume");
                        list_item.set_child(Some(&list_child));
                    }
                    "BoxSet" | "Tag" | "Genre" => {
                        let tu_item: TuItem = glib::object::Object::new();
                        tu_item.set_id(latest.id.clone());
                        tu_item.set_name(latest.name.clone());
                        let list_child =
                            TuListItem::new(tu_item, latest.latest_type.as_str(), false);
                        list_item.set_child(Some(&list_child));
                    }
                    _ => {}
                }
            }
        });
        imp.listgrid.set_factory(Some(&factory));
        imp.listgrid.set_model(Some(&imp.selection));
        imp.listgrid.set_min_columns(3);
        imp.listgrid.set_max_columns(13);
        imp.listgrid.connect_activate(
            glib::clone!(@weak self as obj => move |gridview, position| {
                let model = gridview.model().unwrap();
                let item = model.item(position).and_downcast::<glib::BoxedAnyObject>().unwrap();
                let result: std::cell::Ref<Latest> = item.borrow();
                let window = obj.root().and_downcast::<Window>().unwrap();
                if result.latest_type == "Movie" {
                    window.set_title(&result.name);
                    let item_page = MoviePage::new(result.id.clone(),result.name.clone());
                    item_page.set_tag(Some(&result.name));
                    window.imp().homeview.push(&item_page);
                } else if result.latest_type == "Series" {
                    window.set_title(&result.name);
                    let item_page = ItemPage::new(result.id.clone(),result.id.clone());
                    item_page.set_tag(Some(&result.name));
                    window.imp().homeview.push(&item_page);
                } else {
                    window.set_title(&result.name);
                    let item_page = SingleListPage::new(result.id.clone(),"".to_string(),&result.latest_type, obj.imp().id.get().cloned());
                    item_page.set_tag(Some(&result.name));
                    window.imp().homeview.push(&item_page);
                }
                std::env::set_var("HOME_TITLE", &result.name);
            }),
        );
        self.update(&update_order).await;
    }

    pub async fn update(&self, order: &str) {
        let order = order.to_owned();
        let scrolled = self.imp().listscrolled.get();
        let include_item_types = self.get_include_item_types().to_owned();
        scrolled.connect_edge_overshot(glib::clone!(@weak self as obj => move |_, pos| {
            if pos == gtk::PositionType::Bottom {
                let order = order.clone();
                let spinner = obj.imp().spinner.get();
                spinner.set_visible(true);
                let store = obj.imp().selection.model().unwrap().downcast::<gio::ListStore>().unwrap();
                let id = obj.imp().id.get().expect("id not set").clone();
                let offset = obj.imp().selection.model().unwrap().n_items();
                let listtype = obj.imp().listtype.get().unwrap().clone();
                spinner.set_visible(true);
                let parentid = obj.imp().parentid.borrow().clone();
                let include_item_types = include_item_types.clone();
                let list_results = spawn_tokio(async move {
                    if let Some(parentid) = parentid {
                        get_inlist(parentid.to_string(), offset.to_string(), &listtype, &id, &order).await.unwrap()
                    } else {
                        get_list(id.to_string(), offset.to_string(), &include_item_types, &listtype, &order).await.unwrap()
                    }
                });
                spawn(glib::clone!(@weak store=> async move {
                        let list_results = list_results.await;
                        for result in list_results.items {
                            let object = glib::BoxedAnyObject::new(result);
                            store.append(&object);
                        }
                        spinner.set_visible(false);
                }));
            }
        }));
    }

    pub async fn sortorder(&self, order: &str) {
        let order = order.to_owned();
        let spinner = self.imp().spinner.get();
        let store = self
            .imp()
            .selection
            .model()
            .unwrap()
            .downcast::<gio::ListStore>()
            .unwrap();
        let id = self.imp().id.get().expect("id not set").clone();
        let listtype = self.imp().listtype.get().unwrap().clone();
        spinner.set_visible(true);
        let parentid = self.imp().parentid.borrow().clone();
        let include_item_types = self.get_include_item_types().to_owned();
        let list_results = spawn_tokio(async move {
            if let Some(parentid) = parentid {
                get_inlist(parentid.to_string(), 0.to_string(), &listtype, &id, &order).await
            } else {
                get_list(
                    id.to_string(),
                    0.to_string(),
                    &include_item_types,
                    &listtype,
                    &order,
                )
                .await
            }
        })
        .await
        .unwrap();
        spawn(glib::clone!(@weak store,@weak self as obj=> async move {
                store.remove_all();
                for result in list_results.items {
                    let object = glib::BoxedAnyObject::new(result);
                    store.append(&object);
                }
                spinner.set_visible(false);
        }));
    }

    pub fn get_include_item_types(&self) -> &str {
        let c = self.imp().collectiontype.get().unwrap();
        match c.as_str() {
            "movies" => "Movie",
            "tvshows" => "Series",
            "music" => "MusicAlbum",
            _ => "Movie, Series",
        }
    }
}
