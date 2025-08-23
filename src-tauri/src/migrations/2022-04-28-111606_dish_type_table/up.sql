CREATE TABLE dish_types (
  id INTEGER NOT NULL PRIMARY KEY,
  name_en VARCHAR NOT NULL,
  name_cn VARCHAR NOT NULL
);

INSERT INTO dish_types (name_en, name_cn) VALUES
('Starters', '头盘'),
('Soup Ribs', '汤 排骨'),
('C Wings', '鸡翅'),
('Seafood', '海鲜'),
('B.Bean S', '黑椒汁'),
('Cant. Szech', '广 川'),
('Roast Duck', '鸭'),
('Kung Po Satay', '宫保 沙嗲'),
('Fried Rice', '炒饭'),
('Chow Mein', '炒面'),
('Plum Ginger', '梅 姜'),
('King Prawn', '大虾'),
('Chicken', '鸡'),
('Beef', '牛'),
('Curry', '咖喱'),
('Veg', '斋'),
('Foo Yung', '芙蓉'),
('Sweet & Sour', '古老'),
('Chop Suey', '什水'),
('Deep Fried', '炸'),
('English', '英餐'),
('Extra', '附加'),
('Lunch', '午餐'),
('Sweet & Drink', '甜品 饮料'),
('Set Dinner', '套餐'),
('Others', '其它')