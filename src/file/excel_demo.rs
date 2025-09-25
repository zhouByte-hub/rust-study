/**
 *  calamine = "0.30.1"
 *
 * 一个纯 Rust 编写的 Excel/OpenDocument 电子表格文件读取/反序列化器。
 */

#[cfg(test)]
mod excel_test {
    use calamine::RangeDeserializerBuilder;
    use calamine::Reader;
    use calamine::Xlsx;
    use calamine::open_workbook;

    /**
     * 读取 Excel 文件中的所有工作表名称
     */
    #[test]
    fn read_sheet_test() {
        let work_book: Xlsx<_> = open_workbook("src/file/users.xlsx").unwrap();
        work_book.sheet_names().iter().for_each(|name| {
            println!("{}", name);
        });
    }

    #[test]
    fn read_table_test() {
        let mut work_book: Xlsx<_> = open_workbook("src/file/users.xlsx").unwrap();
        let sheet_name = &work_book.sheet_names()[0];
        let range = work_book.worksheet_range(sheet_name.as_str()).unwrap();

        // 读取一行
        for row in range.rows() {
            // 读取每列
            for cell in row {
                println!("{} ", cell);
            }
        }
    }

    // 鸡肋
    #[test]
    fn read_table_test_1() {
        let mut work_book: Xlsx<_> = open_workbook("src/file/users.xlsx").unwrap();
        let sheet_name = &work_book.sheet_names()[0];
        let range = work_book.worksheet_range(sheet_name.as_str()).unwrap();

        let mut range_deserializer = RangeDeserializerBuilder::new().from_range(&range).unwrap();

        if let Some(item) = range_deserializer.next() {
            let (label, value): (String, String) = item.unwrap();
            println!("{}: {}", label, value);
        } else {
            println!("No data");
        }
    }

    #[test]
    fn read_table_header_test() {
        let mut work_book: Xlsx<_> = open_workbook("src/file/users.xlsx").unwrap();
        let sheet_name = &work_book.sheet_names()[0];
        let range = work_book.worksheet_range(sheet_name.as_str()).unwrap();

        // 不带标题行
        let range = RangeDeserializerBuilder::with_headers(&[
            "用户名",
            "性别",
            "年龄",
            "出生年月",
            "家庭住址",
        ])
        .from_range(&range)
        .unwrap();

        for item in range {
            let (username, sex, age, birth, address): (String, String, String, String, String) =
                item.unwrap();
            println!("{}: {}: {}: {}: {}", username, sex, age, birth, address);
        }
    }

    #[test]
    fn read_table_row_test() {
        let mut work_book: Xlsx<_> = open_workbook("src/file/users.xlsx").unwrap();
        let sheet_name = &work_book.sheet_names()[0];
        let range = work_book.worksheet_range(sheet_name.as_str()).unwrap();

        for row in range.rows() {
            println!("row={:?}, row[0]={:?}", row, row[0]);
        }
    }
}
