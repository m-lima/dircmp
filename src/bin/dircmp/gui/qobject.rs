use qmetaobject::{QAbstractListModel, QObject};

#[derive(Default, qmetaobject::QObject)]
struct Comparator {
    base: qmetaobject::qt_base_class!(trait QObject),
    compare: qmetaobject::qt_method!(fn(&mut self, left: String, right: String)),
    results: qmetaobject::qt_signal!(list: std::cell::RefCell<Results>),
    done: qmetaobject::qt_signal!(),
    error: qmetaobject::qt_signal!(message: String),
}

impl Comparator {
    fn compare(&mut self, left: String, right: String) {
        // let send_results = qmetaobject::queued_callback(move |results| {
        //     // let results = results.into();
        //     self.results(results);
        // });

        let send_error = qmetaobject::queued_callback(move |message| {
            self.error(message);
        });

        std::thread::spawn(move || {
            match dircmp::compare(
                std::path::PathBuf::from(left),
                std::path::PathBuf::from(right),
            ) {
                Ok((dir, _)) => {
                    let (_, list) = dir.decompose();
                    // self.results.borrow_mut().set(list);
                }
                Err(e) => send_error(e.to_string()),
            }
        });
    }
}

#[derive(Default, Clone, qmetaobject::SimpleListItem)]
struct Entry {
    pub path: String,
}

#[derive(Default, qmetaobject::QObject)]
struct Results {
    base: qmetaobject::qt_base_class!(trait QAbstractListModel),
    list: Vec<dircmp::Entry>,
}

impl Results {
    fn clear(&mut self) {
        qmetaobject::QAbstractListModel::begin_reset_model(self);
        self.list.clear();
        qmetaobject::QAbstractListModel::end_reset_model(self);
    }

    fn set(&mut self, list: Vec<dircmp::Entry>) {
        qmetaobject::QAbstractListModel::begin_reset_model(self);
        self.list = list;
        qmetaobject::QAbstractListModel::end_reset_model(self);
    }
}

impl QAbstractListModel for Results {
    fn row_count(&self) -> i32 {
        self.list.len().try_into().unwrap_or(i32::MAX)
    }

    fn data(&self, index: qmetaobject::QModelIndex, role: i32) -> qmetaobject::QVariant {
        let index = usize::try_from(index.row()).unwrap_or(0);

        self.list
            .get(index)
            .and_then(|item| {
                if role == qmetaobject::USER_ROLE {
                    Some(qmetaobject::QVariant::from(qmetaobject::QString::from(
                        item.path().to_string_lossy().into_owned(),
                    )))
                } else if role == qmetaobject::USER_ROLE + 1 {
                    Some(qmetaobject::QVariant::from(qmetaobject::QString::from(
                        item.status().as_str(),
                    )))
                } else {
                    None
                }
            })
            .unwrap_or_default()
    }

    fn role_names(&self) -> std::collections::HashMap<i32, qmetaobject::QByteArray> {
        [
            (
                qmetaobject::USER_ROLE,
                qmetaobject::QByteArray::from("path"),
            ),
            (
                qmetaobject::USER_ROLE,
                qmetaobject::QByteArray::from("status"),
            ),
        ]
        .into_iter()
        .collect()
    }
}

#[derive(qmetaobject::QEnum)]
#[repr(u8)]
enum Status {
    Same,
    Moved,
}
