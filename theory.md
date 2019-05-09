Teorie
=====

Zúčtovací období trvá od prvního do posledního dne v měsíci, nezávisle na příjmech.

Do disponibilní částky nejsou započítány takové výdaje, které jsou pravidelné a neměnné, tedy:

- nájem a poplatky
- elektřina
- internet
- spoření

Do disponibilní částky je započítáno jídlo, jelikož zde existuje obrovské možné rozpětí výdajů. Stejně tak sem spadají jakékoli jiné výdaje neuvedené výše.

Do disponibilní částky nejsou započítány pravidelné a neměnné příjmy, tedy výplata. Do disponibilní částky jsou započítány nepravidelné a proměnné příjmy.

Money management
----------------

Základním prvkem je decoupling příjmů a výdajů. Je vytvořen koncept disponibilní částky, jež je pevná a nezávislá na příjmech.

V úvahu jsou brány dva účty: běžný a spořící. Na začátku zúčtovacího období je ze spořícího účtu na běžný převedena celková disponibilní částka pro toto období, přičemž jakýkoli zbytek z celkové disponibilní částky za minulé zúčtovací období je ponechán na běžném účtu jako cushion.

Když přijde výplata, jsou zaplaceny výdaje nepočítané do disponibilní částky, vše ostatní je následně přesunuto na spořící účet.

(více viz *Firefly III* dokumentaci)

Sumarizační výpočty
-------------------

Sumarizační výpočty jsou sadou proměnných, jež jsou kalkulovány pro každý měsíc zvlášť, na základě dat z tohoto měsíce. Jejich účelem je:

- Nabídnout snadný vhled do finanční situace v rámci daného měsíce (utrácím více něž bych měl? mohu si dovolit výrazně větší výdaj? kolik dní musím hladovět, abych si mohl dovolit *x*?)

- Poskytnout přehled o finančním chování za uplynulé měsíce.

Je definována *celková disponibilní částka* pro dané zúčtovací období. Na jejím základě je spočtena *počáteční povolená útrata per day* tak, že:

    PPÚPD = CDČ / (počet dní v měsíci)

Tedy, pokud každý den utratím přesně *PPŮPD*, pak na konci zúčtovacího období zústane z *počáteční disponibilní částky* 0,-.

Pro každý den se počítá *suma denní útraty* jako triviální suma všech částek pro všechny transakce v daném dni v měsíci v roce.

Pro daný den v měsíci *cd* (číselně rovný počtu uplynulých dní v měsíci) a zbývající dni v měsíci *rd* můžeme spočíst následující proměnné.

*Dosavadní útratu* jako (kde *SDÚ* je suma denní útraty pro každý den v daném měsíci):

    DÚ = sum(SDÚ)

*Zbývající disponibilní částku* jako:

    ZDČ = CDČ - DÚ

*Průměrnou denní útratu*, která by vždy měla být menší než *PPŮPD*, jako:

    PDÚ = DÚ / cd

*Aktuální povolenou útratu per day* jako:

    APÚPD = ZDČ / rd

*Potenciální zůstatek* jako:

    PZ = ZDČ - (rd * PDÚ)

*Saldo* je rozdíl mezi tím, co mělo touto dobou být utraceno a co skutečně bylo utraceno. Vypočte se jako:

    S = (cd * PPÚPD) - DÚ

Výše popsané vzorce platí pro aktuální měsíc. Pro uplynulé měsíce se vzorce pozměňují tak, že *aktuální den v měsíci* je nahrazen *posledním dnem v měsíci*.

Aplikační specifikace
---------------------

Webová aplikace v Rustu, PostgreSQL. WebUI. Post, Redirect, Get.

- Vše co jde bude prováděno server-side: obarvování výdajů (červená/zelená), plnění selectů hodnotami...

- Pokud je aktivní jakýkoliv input a stiskne se klávesa Enter, vezmou se hodnoty všech inputů (bez ohledu na to, zda byly změněny) a pošlou se v JSON na server, kde proběhne aktualizace hodnot v databázi, případně zapsání nových hodnot.

- Každému řádku denních výdajů musí být přiřazeno ID odpovídající PK v databázi.

- Peníze jsou v databázi reprezentovány jako i64 vyjadřující miliontiny koruny (CZK), integer je signed - záporné hodnoty vyjadřují příjmy (!), v aplikaci jsou peníze reprezntovány jako f64 korun.

- Povolená denní útrata bude uložena v databázi ve formě (*PDÚ*, *PlatnostOd*), kde PDÚ je částka povolené denní útraty ve formě popsané v předchozím odstavci; PlatnostOd je datum (měsíc a rok) od kterého začíná platnost této částky PDÚ. Částka platí od tohoto termínu do nadcházejícího, případně indefinitivně (dokud není vložena nová hodnota), pokud v databázi není záznam s vyšší hodnotou *PlatnostOd*.

- Pro účely pozdější analýzy je potřeba ukládat výsledky sumarizačních výpočtů do databáze. **Je potřeba dospecifikovat, jak přesně toto má fungovat.**