# Exercise 2
Cho 1 chuỗi str Slice như dưới đây. Nhập 1 từ bất kỳ từ bàn phím, in ra số lượng từ này xuất hiện trong chuỗi đã cho.
 `https://ars.els-cdn.com/content/image/1-s2.0-S0960982203005347-mmc6.txt`
*Note: Nâng cao hơn : Tìm kiếm không phân biêt chữ hoa thường, theo dạng regex.

# Idea
Basic
   - The idea here is we try to make use of `windows` method. And solve the problem with the idea that is introduced at the previous exercise `sublist`.
Advanced
   - Relative Search: make use of `to_lowercase` method before searching.
   - Regex Search: make use of the `regex` dependency