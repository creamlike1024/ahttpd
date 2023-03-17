use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>
}
// 参数值可以为单个值，也可以为多个值，所以使用枚举类型
#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),   // 多个值时，使用动态数组存储
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        // 示例： "search=abc&sort=1&order=desc"
        let mut data = HashMap::new();
        for sub_str in s.split('&') {   // split() 方法用于分割字符串，返回一个迭代器
            let mut key = sub_str;
            let mut val = "";
            // if let 语句用于 Option 处理
            if let Some(i) = sub_str.find('=') {    // find() 方法用于查找字符串中第一个匹配的索引，如果没有匹配的则返回 None
                key = &sub_str[..i];
                val = &sub_str[i+1..];  // i+1 是为了去除 =
            }
            // or_insert() 方法用于插入值，如果 key 不存在则插入，如果 key 存在则不做任何操作
            // and_modify() 方法用于修改值，如果 key 存在则修改，如果 key 不存在则不做任何操作
            // entry() 方法用于获取 key 对应的值，如果 key 不存在则插入一个默认值
            data.entry(key)
                // 如果 key 存在，则修改其值
                .and_modify(|existing: &mut Value| match existing {
                    //and_modify 会将 Hashmap 中的值作为参数传入闭包中，所以这里的 existing 是 &mut Value 类型，所以需要解引用
                    // 闭包后使用 match 语句或函数 {} 来处理
                    Value::Single(prev_val) => {
                        // 如果是单个值，则将其转换为多个值
                        *existing = Value::Multiple(vec![prev_val, val]);
                    }
                    // 已经是多个值，则直接添加
                    Value::Multiple(vec) => vec.push(val),
                })
                .or_insert(Value::Single(val)); // 如果 key 不存在，则插入一个单个值
        }
        QueryString { data }
    }
}