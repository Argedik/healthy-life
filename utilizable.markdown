<!-- # Başlık 1
## Başlık 2
### Başlık 3
*italik*
**kalın**
***kalın ve italik***
- madde 1
- madde 2
  1. madde 1
  2. madde 2
[örnek metin](https://www.markdownguide.org/)
`kod`
ünlem ile başlamalı [Alternatif Metin](https://chatgpt.com/c/6729a56a-2b30-800d-aff2-571c779f44f4) 

-->

### cargo kodları
- ***new*** proje oluşturma
- ***build*** proje derleme
- ***run*** projeyi derler ve çalıştırır
- ***check*** hata kontrolü yapılabiliyor
- ***cargo build --release*** proje yayınlama (*target/debug yerine target/release klasörü içine dosyalar oluşturulacaktır.*)
- main.rs dosyasını src içine ve target dosyasına da build ve debug dosyalarını kaydetmeyi sağlıyor


***&mut ile mut arasındaki fark;*** **mut** main fonksiyonu içinde değiştirilebilir. **&mut** değişkenin bellekte referans olarak aldığı değişkenin değerini değiştiriyor.
***struct*** <u>Veri türleri başlığı altına yer alır.</u> Javascriptte class içine tanımlanan constructor örneğine benzer ya da es6 öncesi obje tanımlamasına benzer bir yapı. c#, c, c++, go gibi dillerde de kullanılıyor. 
***impl*** 

***cargo modules structure*** **cargo install cargo-modules** ile projemizin klasör ve dosya yapılandırmasına göz atabiliyoruz.
=======
---

***Appendix A: Keywords***
https://rust-book.cs.brown.edu/appendix-01-keywords.html
as - perform primitive casting, disambiguate the specific trait containing an item, or rename items in use statements
async - return a Future instead of blocking the current thread
await - suspend execution until the result of a Future is ready
break - exit a loop immediately
const - define constant items or constant raw pointers
continue - continue to the next loop iteration
crate - in a module path, refers to the crate root
dyn - dynamic dispatch to a trait object
else - fallback for if and if let control flow constructs
enum - define an enumeration
extern - link an external function or variable
false - Boolean false literal
fn - define a function or the function pointer type
for - loop over items from an iterator, implement a trait, or specify a higher-ranked lifetime
if - branch based on the result of a conditional expression
impl - implement inherent or trait functionality
in - part of for loop syntax
let - bind a variable
loop - loop unconditionally
match - match a value to patterns
mod - define a module
move - make a closure take ownership of all its captures
mut - denote mutability in references, raw pointers, or pattern bindings
pub - denote public visibility in struct fields, impl blocks, or modules
ref - bind by reference
return - return from function
Self - a type alias for the type we are defining or implementing
self - method subject or current module
static - global variable or lifetime lasting the entire program execution
struct - define a structure
super - parent module of the current module
trait - define a trait
true - Boolean true literal
type - define a type alias or associated type
union - define a union; is only a keyword when used in a union declaration
unsafe - denote unsafe code, functions, traits, or implementations
use - bring symbols into scope
where - denote clauses that constrain a type
while - loop conditionally based on the result of an expression

abstract
become
box
do
final
macro
override
priv
try
typeof
unsized
virtual
yield

---

***Data Types*** **Scalar Types** ve **Compound Types** olmak üzere iki alt kümeye ayrılıyor.
***Scalar Types*** Bir skaler veri tipi tek bir değeri temsil eder. 4 temel skaler tipi vardır. 
**Boolean** *(false, true)*, 
**Integer** *(*Integer Types* (Signed (i8, i16, i32, i64, i128, isize), Unsigned (u8, u16, u32, u64, u128, usize) boyutları da tanımlanan rakamlarla aynı usize ve isize arch boyutta olduğu belirtiliyor. "https://rust-book.cs.brown.edu/ch03-02-data-types.html"), *Integer Literals* (Decimal	98_222, Hex	0xff ,Octal	0o77 ,Binary	0b1111_0000 ,Byte (u8 only)	b'A'))*, 
**Floating-Point Types** *(f32, f64)*, 
**Character Type** *('')*,

***Compound Types*** Bileşik tipler birden fazla değeri tek bir tipte gruplayabilir. İki bileşik tür vardır.
**Tuple Type** Çeşitli türlerdeki bir dizi değeri tek bir bileşik türde gruplamanın genel bir yoludur. *(let tup: (i32, f64, u8) = (500, 6.4, 1); ) ((let tup = (500, 6.4, 1); let (x, y, z) = tup;) . (nokta) ile de değerlere erişilebiliyor. (let x: (i32, f64, u8) = (500, 6.4, 1); let five_hundred = x.0; (0 yerine 1, 2 de yazılabilir.)))*
**Array Type** Birden fazla değerden oluşan bir koleksiyona sahip olmanın bir başka yolu da dizidir. Tuple'ın aksine, bir dizinin her elemanı aynı türde olmalıdır. Sabit uzunluktadır. *(let a: [i32; 5] = [1, 2, 3, 4, 5]; let a = [1, 2, 3, 4, 5]; let first = a[0];)*

***Threads*** **Race condition**, **Deadlock**, **Channels**

***Docker*** 
*docker-compose up --build* Tüm servisleri çalıştırır.
*docker-compose up --build web* sadece web servisini çalıştırır.
*docker ps -a* tüm çalışan container ları listeler
*docker start <container-id>* belirli container durduysa tekrar başlatır


***trunk***
*kendisinde scss desteği var. html dosyasına dahil ettiğimiz css dosyasının tag attributlerine data-trunk eklememiz ve rel attribütüne scss yazmamız yeterli*
trunk serve --adress 0.0.0.0 --port 8081
cargo install --force trunk *trunk'ı güncelle*
cargo install --force wasm-bindgen-cli
rustup update *rust'ı güncelle*
rustup target list --installed *rustup target add wasm32-unknown-unknown ile wasm dosya yolunun olup olmadığını kontrol et*
--open bu seçenek ile çalıştırıldıktan sonra otomatik browserı açıyor
--watch belirtilen dosyalarda değişiklik yaptıldığında otomatik build almasını sağlıyor
clean
***Trunk.toml***5478

620
1180
1610
3600


***Cargo***
clean

***Cargo.toml***
**web-sys** dist klasörüne wasm ve html dosyası oluşturuluyor.

***Ownership / stack and heap***
Stack, değerleri aldığı sırayla depolar ve ters sırayla kaldırır. Buna son giren ilk çıkar denir. 
Veri eklemeye yığına itme denir ve veriyi kaldırmaya yığından çıkarma denir.
Yığında depolanan tüm verilerin bilinen, sabit bir boyutu olmalıdır. Derleme zamanında bilinmeyen bir boyuta sahip olan veya boyutu değişebilecek veriler bunun yerine yığında depolanmalıdır.
*The stack stores values in the order it gets them and removes the values in the opposite order. This is referred to as last in, first out.*
*Adding data is called pushing onto the stack, and removing data is called popping off the stack*
*All data stored on the stack must have a known, fixed size. Data with an unknown size at compile time or a size that might change must be stored on the heap instead.*

Heap, bellek üzerinde alan tahsis etmeyi sağlıyor.

**Ownership Rules**
- Each value in Rust has an owner. (Rust'taki her değerin bir sahibi vardır.)
- There can only be one owner at a time. (Aynı anda yalnızca bir sahibi olabilir.)
- When the owner goes out of scope, the value will be dropped.
(Sahip kapsam dışına çıktığında değer düşecektir.)
- Rust i32 için copy işlemi [uygulayabilirken](https://rust-book.cs.brown.edu/ch04-03-fixing-ownership-errors.html#fixing-an-unsafe-program-aliasing-and-mutating-a-data-structure), string için copy işlemi uygulayamaz.
- Aynı veriye sahip 2 değiştirilebilir referans, [aynı anda kullanılamaz.](https://rust-book.cs.brown.edu/ch04-03-fixing-ownership-errors.html#fixing-an-unsafe-program-aliasing-and-mutating-a-data-structure)

***References and Borrowing***
- & bir değişkene referams ya da borrow uygulayabiliyoruz
- ' bir değişkeni sonsuza kadar string literal olur
- Rc akıllı işaretçi referansı

*String::from ile .to_string() farkı; String::from yalnızca string tipler için .to_string() tüm tipler için*

**unsafe { *x += *y; } iki farklı dizeyi güvenli olmayan bloklar sayesinde error almadan birleştirebiliriz.**
- & işareti ile bir değişkene referams ya da borrow uygulayabiliyoruz

***Errors***
Proc macros not being expanded / rust-analyzer: Restart Server
