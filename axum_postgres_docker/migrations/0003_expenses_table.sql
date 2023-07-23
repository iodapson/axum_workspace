CREATE TABLE expenses (
  expense_id INT PRIMARY KEY,
  expense_amount DECIMAL(10,2) NOT NULL,
  expense_item_name VARCHAR NOT NULL,
  expense_date DATE NOT NULL,
  category_id INT NOT NULL,
  FOREIGN KEY (category_id) REFERENCES expense_categories(category_id)
);