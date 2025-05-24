# खिया (Khiya)

अङ्ग्रेजीमै Rust लेख्दा लेख्दै वाक्क भइसकेको हो? दिमागले नेपालीमै सोच्छ, तर हातले चाहिँ `fn`,
`let`, `return` टाइप गर्नुपर्छ? हो भने भेट्नुस् **खिया।** खियाले Rust लाई नेपाली बनाउँछ। 

यो मजाक मात्रै होइन, यो भाषिक पहुँचको सवाल हो। अंग्रेजी आउनुपर्छ भन्ने डर हटाऔं। नेपालीमा
सोच्ने मान्छेले कोड पनि नेपालीमै लेख्न पाइन्छ। हामीमध्ये कतिलाई अङ्ग्रेजीमा लेख्दा लेख्दा अङ्ग्रेजी
अलि धेरै आउने पनि हुन्छ, तिनीहरूले दुःख मान्नुपर्दैन। खिया, is fully compatible with
standard English-Rust. So you can easily mix and match both in your projects,
using the power of Rust with the awesomeness of Nepali.

Here's an example of what can be achieved with Khiya:

### trait and impl ()

```rust
rouille::rouille! {
    utilisons std::collections::Dictionnaire comme Dico;

    convention CléValeur {
        fonction écrire(&soi, clé: Chaîne, valeur: Chaîne);
        fonction lire(&soi, clé: Chaîne) -> PeutÊtre<&Chaîne>;
    }

    statique mutable DICTIONNAIRE: PeutÊtre<Dico<Chaîne, Chaîne>> = Rien;

    structure Concrète;

    réalisation CléValeur pour Concrète {
        fonction écrire(&soi, clé: Chaîne, valeur: Chaîne) {
            soit dico = dangereux {
                DICTIONNAIRE.prendre_ou_insérer_avec(Défaut::défaut)
            };
            dico.insérer(clé, valeur);
        }
        fonction lire(&soi, clé: Chaîne) -> Résultat<PeutÊtre<&Chaîne>, Chaîne> {
            si soit Quelque(dico) = dangereux { DICTIONNAIRE.en_réf() } {
                Bien(dico.lire(&clé))
            } sinon {
                Arf("fetchez le dico".vers())
            }
        }
    }
}
```

### Other examples

See the [examples](./examples/src/main.rs) to get a rough sense of the whole
syntax.

## Other languages

- Dutch: [roest](https://github.com/jeroenhd/roest)
- German: [rost](https://github.com/michidk/rost)
- Polish: [rdza](https://github.com/phaux/rdza)
- Italian: [ruggine](https://github.com/DamianX/ruggine)
- Russian: [Ржавый](https://github.com/Sanceilaks/rzhavchina)
- Esperanto: [rustteksto](https://github.com/dscottboggs/rustteksto)
- Toki Pona: [jaki kiwen](https://github.com/jgcodes2020/jaki-kiwen)
- Hindi: [zung](https://github.com/rishit-khandelwal/zung)
- Hungarian: [rozsda](https://github.com/jozsefsallai/rozsda)
- Chinese: [xiu (锈)](https://github.com/lucifer1004/xiu)
- Spanish: [rustico](https://github.com/UltiRequiem/rustico)
- Korean: [Nok (녹)](https://github.com/Alfex4936/nok)
- Finnish: [ruoste](https://github.com/vkoskiv/ruoste)
- Arabic: [sada](https://github.com/LAYGATOR/sada)
- Turkish: [pas](https://github.com/ekimb/pas)
- Vietnamese: [gỉ](https://github.com/Huy-Ngo/gir)
- Japanese: [sabi (錆)](https://github.com/yuk1ty/sabi)
- Danish: [rust?](https://github.com/LunaTheFoxgirl/rust-dk)
- Marathi: [gan̄ja](https://github.com/pranavgade20/ganja)
- Romanian: [rugină](https://github.com/aionescu/rugina)
- Czech: [rez](https://github.com/radekvit/rez)
- Ukrainian: [irzha](https://github.com/brokeyourbike/irzha)
- Bulgarian: [ryzhda](https://github.com/gavadinov/ryzhda)
- Slovak: [hrdza](https://github.com/TheMessik/hrdza)
- Catalan: [rovell](https://github.com/gborobio73/rovell)
- Corsican: [rughjina](https://github.com/aldebaranzbradaradjan/rughjina)
- Indonesian: [karat](https://github.com/annurdien/karat)
- Lithuanian: [rūdys](https://github.com/TruncatedDinosour/rudys)
- Greek: [skouriasmeno](https://github.com/devlocalhost/skouriasmeno)
- Thai: [sanim (สนิม)](https://github.com/korewaChino/sanim)
- Swiss: [roeschti](https://github.com/Georg-code/roeschti)
- Swedish: [rost](https://github.com/vojd/rost/)
- Croatian: [hrđa](https://github.com/njelich/hrdja)
- Persian: [zangar (زنگار)](https://github.com/ui-ce/zangar)
- Malagasy: [arafesina](https://github.com/luckasRanarison/arafesina)
- Latin: [ferrugo](https://github.com/pianoman911/ferrugo)
- Norwegian: [korrosjon](https://github.com/datagutt/korrosjon)
- Estonian: [rooste](https://github.com/hanshs/rooste)
- Kannada [tukku (ತುಕ್ಕು)](https://github.com/sanathNU/tukku.git)
- All of the above: [unirust](https://github.com/charyan/unirust)

## license (अनुमतिपत्र)

[WTFPL](http://www.wtfpl.net/)
