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

/**
 * rust_xlsxwriter = "0.90.1"
 *
 */
#[cfg(test)]
mod excel_write_test {
    use rust_xlsxwriter::{
        Color, Format, FormatAlign, FormatBorder, workbook::Workbook, worksheet::Worksheet,
    };

    #[test]
    fn excel_test_1() {
        let mut workbook = Workbook::new();
        let mut worksheet = Worksheet::new();

        let title = Format::new()
            .set_bold()
            .set_font_size(16)
            .set_font_color(Color::White)
            .set_background_color(Color::Black)
            .set_align(FormatAlign::Center)
            .set_align(FormatAlign::VerticalCenter)
            .set_border(FormatBorder::Thin);

        /*  合并表格
           worksheet.merge_range(first_row, first_col, last_row, last_col, string, format)
        */
        worksheet
            .merge_range(0, 0, 0, 4, "用户信息", &title)
            .unwrap();

        // 设置表头格式
        let header_format = Format::new()
            .set_bold()
            .set_font_color(Color::White)
            .set_background_color(Color::Black)
            .set_align(FormatAlign::Center)
            .set_border(FormatBorder::Thin);
        // 写入表头
        let headers = ["用户名", "性别", "年龄", "出生年月", "家庭住址"];
        for (index, item) in headers.iter().enumerate() {
            worksheet
                .write_string_with_format(0, index as u16, *item, &header_format)
                .unwrap();
        }

        // 数据格式
        let string_data_format = Format::new()
            .set_border(FormatBorder::Thin)
            .set_align(FormatAlign::Center);
        let birthday_format = Format::new()
            .set_num_format("yyyy-mm-dd")
            .set_border(FormatBorder::Thin)
            .set_align(FormatAlign::Center);

        let data = [
            ("张三", "男", "25", "2000-01-01", "中国"),
            ("李四", "女", "30", "1995-05-15", "中国"),
        ];
        for (row, (name, sex, age, birth, address)) in data.iter().enumerate() {
            let row = (row + 1) as u32;
            worksheet
                .write_string_with_format(row, 0, name.to_string(), &string_data_format)
                .unwrap();
            worksheet.write_string(row, 1, sex.to_string()).unwrap();
            worksheet.write_string(row, 2, age.to_string()).unwrap();
            worksheet
                .write_string_with_format(row, 3, birth.to_string(), &birthday_format)
                .unwrap();
            worksheet.write_string(row, 4, address.to_string()).unwrap();
        }

        // 自动调整列宽
        worksheet.autofit();
        workbook.push_worksheet(worksheet);
        workbook.save("src/file/users_1.xlsx").unwrap();
    }
}
