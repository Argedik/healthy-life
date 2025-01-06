***Renderer***
Burada tıpkı React da olduğu gibi <App> yazıyoruz App yerine herhangi bir isimlendirme olabilir. App elementini bularak içine tüm diğer yazdığımız html kodlarını entegre ediyor.

Dökümanında aşağıdaki gibi bir yapı var 

```
pub struct Renderer<COMP>
where
    COMP: BaseComponent + 'static,
{ /* private fields */ }
```