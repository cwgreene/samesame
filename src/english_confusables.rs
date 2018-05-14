extern crate rand;
use self::rand::Rng;

use std::string::String;
use std::collections::HashMap;

pub fn map(input: String) -> String {
        let mut confusables = HashMap::new();
    
    confusables.insert('A', vec!['\u{FF21}','\u{1D400}','\u{1D434}',
        '\u{19468}','\u{1D49C}','\u{1D4D0}','\u{1D504}','\u{1D538}','\u{1D56C}','\u{1D5A0}','\u{1D5D4}','\u{1D608}',
        '\u{1D63C}','\u{1D670}','\u{1D00}','\u{0391}','\u{1D6A8}','\u{1D6E2}','\u{1D71C}','\u{1D756}','\u{1D790}',
        '\u{0410}','\u{13AA}','\u{15C5}','\u{A4EE}','\u{102A0}']);

    confusables.insert('B', vec!['\u{FF22}','\u{212C}','\u{1D401}','\u{1D435}','\u{1D469}','\u{1D4D1}','\u{1D505}','\u{1D609}','\u{1D63D}','\u{1D671}','\u{0392}','\u{1D6A9}','\u{1D6E3}','\u{1D71D}','\u{1D757}','\u{1D791}','\u{0412}','\u{13F4}','\u{15F7}','\u{A4D0}','\u{10282}','\u{102A1}','\u{10301}','\u{A7B4}','\u{0432}']);

    confusables.insert('C', vec!['\u{1F74C}','\u{118F2}','\u{118E9}','\u{FF23}','\u{216D}','\u{2102}','\u{212D}','\u{1D402}','\u{1D436}','\u{1D46A}','\u{1D49E}','\u{1D4D2}','\u{1D56E}','\u{1D5A2}','\u{1D5D6}','\u{1D60A}','\u{1D63E}','\u{1D672}','\u{03FD}','\u{2CA4}','\u{0421}','\u{13DF}','\u{A4DA}','\u{102A2}','\u{10302}','\u{10415}','\u{1051C}','\u{00C7}','\u{04AA}','\u{0187}']);

    confusables.insert('D', vec!['\u{216E}','\u{2145}','\u{1D403}','\u{1D437}','\u{1D46B}','\u{1D49F}','\u{1D4D3}','\u{1D507}','\u{1D53B}','\u{1D56F}','\u{1D5A3}','\u{1D5D7}','\u{1D60B}','\u{1D63F}','\u{1D673}','\u{13A0}','\u{15DE}','\u{15EA}','\u{A4D3}','\u{0110}','\u{00D0}','\u{0189}']);

    confusables.insert('E', vec!['\u{22FF}','\u{FF25}','\u{2130}','\u{1D404}','\u{1D438}','\u{1D46C}','\u{1D4D4}','\u{1D508}','\u{1D53C}','\u{1D570}','\u{1D5A4}','\u{1D5D8}','\u{1D60C}','\u{1D640}','\u{1D674}','\u{0395}','\u{1D6AC}','\u{1D6E6}','\u{1D720}','\u{1D75A}','\u{1D794}','\u{0415}','\u{2D39}','\u{13AC}','\u{A4F0}','\u{118A6}','\u{118AE}','\u{10286}','\u{011A}','\u{0246}','\u{2107}','\u{0510}','\u{13CB}','\u{10401}','\u{0190}']);

    confusables.insert('F', vec!['\u{2131}','\u{1D405}','\u{1D439}','\u{1D46D}','\u{1D4D5}','\u{1D509}','\u{1D53D}','\u{1D571}','\u{1D5A5}','\u{1D5D9}','\u{1D60D}','\u{1D641}','\u{1D675}','\u{A798}','\u{03DC}','\u{1D7CA}','\u{15B4}','\u{A4DD}','\u{118C2}','\u{118A2}','\u{10287}','\u{102A5}','\u{10525}','\u{0191}']);

    confusables.insert('G', vec!['\u{1D406}','\u{1D43A}','\u{1D46E}','\u{1D4A2}','\u{1D4D6}','\u{1D50A}','\u{1D53E}','\u{1D572}','\u{1D5A6}','\u{1D5DA}','\u{1D60E}','\u{1D642}','\u{1D676}','\u{050C}','\u{13C0}','\u{13F3}','\u{A4D6}','\u{01E6}','\u{011E}','\u{01E4}','\u{0193}','\u{0509}']);

    confusables.insert('H', vec!['\u{FF28}','\u{2108}','\u{210C}','\u{210D}','\u{1D407}','\u{1D43B}','\u{1D46F}','\u{1D4D7}','\u{1D573}','\u{1D5A7}','\u{1D5DB}','\u{1D60F}','\u{1D643}','\u{1D677}','\u{0397}','\u{1D6AE}','\u{1D6E8}','\u{1D722}','\u{1D75C}','\u{1D796}','\u{2C8E}','\u{041D}','\u{13BB}','\u{157C}','\u{A4E7}','\u{102CF}','\u{2C67}','\u{04A2}','\u{0126}','\u{04C9}','\u{04C7}']);

    confusables.insert('I', vec!['\u{FF29}','\u{1D6B0}','\u{1D62D}','\u{0406}','\u{1D5A8}','\u{1D425}','\u{FE8D}','\u{FE8E}','\u{1D529}','\u{2110}','\u{2111}','\u{1028A}','\u{2C92}','\u{10309}','\u{2113}','\u{1D724}','\u{0196}','\u{1D798}','\u{0399}','\u{1D695}','\u{1D7CF}','\u{2223}','\u{0627}','\u{FF29}','\u{1D5C5}','\u{1D540}','\u{0031}','\u{1D644}','\u{1D4C1}','\u{10320}','\u{1D43C}','\u{1EE00}','\u{1EE80}','\u{05C0}','\u{1D470}','\u{01C0}','\u{04C0}','\u{16C1}','\u{1D7ED}','\u{1D574}','\u{07CA}','\u{FF4C}','\u{1D6EA}','\u{2D4F}','\u{1D75E}','\u{1D55D}','\u{1D7E3}','\u{05D5}','\u{1E8C7}','\u{1D661}','\u{1D4D8}','\u{1D5DC}','\u{1D7D9}','\u{1D459}','\u{05DF}','\u{2160}','\u{1D610}','\u{0661}','\u{1D48D}','\u{1D591}','\u{FFE8}','\u{1D408}','\u{006C}','\u{06F1}','\u{A4F2}','\u{16F28}','\u{1D678}','\u{1D7F7}','\u{1D4F5}','\u{007C}','\u{217C}','\u{23FD}','\u{1D5F9}']);

    confusables.insert('J', vec!['\u{FF2A}','\u{1D409}','\u{1D43D}','\u{1D471}','\u{1D4A5}','\u{1D4D9}','\u{1D50D}','\u{1D541}','\u{1D575}','\u{1D5A9}','\u{1D5DD}','\u{1D611}','\u{1D645}','\u{1D679}','\u{037F}','\u{0408}','\u{13AB}','\u{148D}','\u{A4D9}','\u{A7B2}','\u{0248}','\u{1499}','\u{0575}','\u{1D6A5}']);

    confusables.insert('K', vec!['\u{212A}','\u{FF2B}','\u{1D40A}','\u{1D43E}','\u{1D472}','\u{1D4A6}','\u{1D4DA}','\u{1D50A}','\u{1D542}','\u{1D576}','\u{1D5AA}','\u{1D5DE}','\u{1D612}','\u{1D646}','\u{1D67A}','\u{039A}','\u{1D6B1}','\u{1D6EB}','\u{1D725}','\u{1D75F}','\u{1D799}','\u{2C94}','\u{041A}','\u{13E6}','\u{16D5}','\u{A4D7}','\u{10518}','\u{2C69}','\u{049A}','\u{049E}','\u{0198}']);

    confusables.insert('L', vec!['\u{216C}','\u{2112}','\u{1D408}','\u{1D43F}','\u{1D473}','\u{1D4DB}','\u{1D50F}','\u{1D543}','\u{1D577}','\u{1D5AB}','\u{1D5DF}','\u{1D613}','\u{1D647}','\u{1D67B}','\u{2CD0}','\u{13DE}','\u{14AA}','\u{A4E1}','\u{118A3}','\u{118B2}','\u{1041B}','\u{10526}','\u{0141}','\u{14B7}','\u{013F}']);

    confusables.insert('M', vec!['\u{FF2D}','\u{216F}','\u{2133}','\u{1D40C}','\u{1D440}','\u{1D474}','\u{1D4DC}','\u{1D510}','\u{1D544}','\u{1D578}','\u{1D5AC}','\u{1D5E0}','\u{1D614}','\u{1D648}','\u{1D67C}','\u{039C}','\u{1D6B3}','\u{1D6ED}','\u{1D727}','\u{1D761}','\u{1D79B}','\u{03FA}','\u{2C98}','\u{041C}','\u{13B7}','\u{15F0}','\u{16D6}','\u{A4DF}','\u{102B0}','\u{10311}','\u{04CD}']);

    confusables.insert('N', vec!['\u{FF2E}','\u{2115}','\u{1D40D}','\u{1D441}','\u{1D475}','\u{1D4A9}','\u{1D4DD}','\u{1D511}','\u{1D579}','\u{1D5AD}','\u{1D5E1}','\u{1D615}','\u{1D649}','\u{1D67D}','\u{039D}','\u{1D6B4}','\u{1D6EE}','\u{1D728}','\u{1D762}','\u{1D79C}','\u{2C9A}','\u{A4E0}','\u{10513}','\u{019D}']);

    confusables.insert('O', vec!['\u{0A66}','\u{0AE6}','\u{0BE6}','\u{0C66}','\u{0CE6}','\u{0ED0}','\u{1040}','\u{0030}','\u{07C0}','\u{09E6}','\u{0B66}','\u{3007}','\u{114D0}','\u{118E0}','\u{1D7CE}','\u{1D7D8}','\u{1D7E2}','\u{1D7EC}','\u{1D7F6}','\u{FF2F}','\u{1D40E}','\u{1D442}','\u{1D476}','\u{1D4AA}','\u{1D4DE}','\u{1D512}','\u{1D546}','\u{1D57A}','\u{1D5AE}','\u{1D5E2}','\u{1D616}','\u{1D64A}','\u{1D67E}','\u{039F}','\u{1D6B6}','\u{1D6F0}','\u{1D72A}','\u{1D764}','\u{1D79E}','\u{2C9E}','\u{041E}','\u{0555}','\u{2D54}','\u{0B20}','\u{0D20}','\u{A4F3}','\u{118B5}','\u{10292}','\u{102AB}','\u{10404}','\u{10516}','\u{01D1}','\u{00D8}','\u{2D41}','\u{01FE}','\u{2296}','\u{229D}','\u{1F100}','\u{1F101}','\u{01A0}','\u{13A4}']);

    confusables.insert('P', vec!['\u{FF30}','\u{2119}','\u{1D40F}','\u{1D443}','\u{1D477}','\u{1D4AB}','\u{1D4DF}','\u{1D513}','\u{1D57B}','\u{1D5AF}','\u{1D5E3}','\u{1D617}','\u{1D64B}','\u{1D67F}','\u{03A1}','\u{1D6B8}','\u{1D6F2}','\u{1D72C}','\u{1D766}','\u{1D7A0}','\u{2CA2}','\u{0420}','\u{13E2}','\u{146D}','\u{A4D1}','\u{10295}','\u{1477}','\u{1486}',]);

    confusables.insert('Q', vec!['\u{211A}','\u{1D410}','\u{1D444}','\u{1D478}','\u{1D4AC}','\u{1D4E0}','\u{1D514}','\u{1D57C}','\u{1D5B0}','\u{1D5E4}','\u{1D618}','\u{1D64C}','\u{1D680}','\u{2D55}']);

    confusables.insert('R', vec!['\u{211B}','\u{211C}','\u{211D}','\u{1D411}','\u{1D445}','\u{1D479}','\u{1D4E1}','\u{1D57D}','\u{1D5B1}','\u{1D5E5}','\u{1D619}','\u{1D64D}','\u{1D681}','\u{01A6}','\u{13A1}','\u{13D2}','\u{1587}','\u{A4E3}']);

    confusables.insert('S', vec!['\u{FF33}','\u{1D412}','\u{1D446}','\u{1D47A}','\u{1D4AE}','\u{1D4E2}','\u{1D516}','\u{1D54A}','\u{1D57E}','\u{1D5B2}','\u{1D5E6}','\u{1D61A}','\u{1D64E}','\u{1D682}','\u{0405}','\u{054F}','\u{13D5}','\u{13DA}','\u{A4E2}','\u{10296}','\u{10420}']);

    confusables.insert('T', vec!['\u{22A4}','\u{27D9}','\u{1F768}','\u{FF34}','\u{1D413}','\u{1D447}','\u{1D47B}','\u{1D4AF}','\u{1D4E3}','\u{1D517}','\u{1D548}','\u{1D57F}','\u{1D5B3}','\u{1D5E7}','\u{1D61B}','\u{1D64F}','\u{1D683}','\u{03A4}','\u{1D6BB}','\u{1D6F5}','\u{1D72F}','\u{1D769}','\u{1D7A3}','\u{2CA6}','\u{0422}','\u{13A2}','\u{A4D4}','\u{118BC}','\u{10297}','\u{102B1}','\u{10315}','\u{2361}','\u{023E}','\u{021A}','\u{01AE}','\u{04AC}','\u{20AE}','\u{0166}']);

    confusables.insert('U', vec!['\u{222A}','\u{22C3}','\u{1D414}','\u{1D448}','\u{1D47C}','\u{1D4B0}','\u{1D4E4}','\u{1D518}','\u{1D54C}','\u{1D580}','\u{1D5B4}','\u{1D5E8}','\u{1D61C}','\u{1D650}','\u{1D684}','\u{054D}','\u{144C}','\u{A4F4}','\u{118B8}','\u{01D3}','\u{0244}','\u{13CC}','\u{1458}','\u{1467}','\u{01B1}','\u{162E}']);

    confusables.insert('V', vec!['\u{2228}','\u{22C1}','\u{0667}','\u{06F7}','\u{2164}','\u{1D415}','\u{1D449}','\u{1D47D}','\u{1D4B1}','\u{1D4E5}','\u{1D519}','\u{1D54D}','\u{1D581}','\u{1D5B5}','\u{1D5E9}','\u{1D61D}','\u{1D651}','\u{1D685}','\u{0474}','\u{2D38}','\u{13D9}','\u{142F}','\u{A4E6}','\u{118A0}','\u{1051D}','\u{143B}','\u{1F708}']);

    confusables.insert('W', vec!['\u{118EF}','\u{118E6}','\u{1D416}','\u{1D44A}','\u{1D47E}','\u{1D4B2}','\u{1D4E6}','\u{1D51A}','\u{1D54E}','\u{1D582}','\u{1D5B6}','\u{1D5EA}','\u{1D61E}','\u{1D652}','\u{1D686}','\u{051C}','\u{13B3}','\u{13D4}','\u{A4EA}','\u{20A9}']);

    confusables.insert('X', vec!['\u{166D}','\u{2573}','\u{10322}','\u{118EC}','\u{FF38}','\u{2169}','\u{1D417}','\u{1D44B}','\u{1D47F}','\u{1D4B3}','\u{1D4E7}','\u{1D51B}','\u{1D54F}','\u{1D583}','\u{1D5B7}','\u{1D5EB}','\u{1D61F}','\u{1D653}','\u{1D687}','\u{03A7}','\u{1D6BE}','\u{1D6F8}','\u{1D732}','\u{1D76C}','\u{1D7A6}','\u{2CAC}','\u{0425}','\u{2D5D}','\u{2D5D}','\u{16B7}','\u{A4EB}','\u{10290}','\u{102B4}','\u{10317}','\u{10527}','\u{A7B3}','\u{04B2}']);

    confusables.insert('Y', vec!['\u{FF39}','\u{1D418}','\u{1D44C}','\u{1D480}','\u{1D4B4}','\u{1D4E8}','\u{1D51C}','\u{1D550}','\u{1D584}','\u{1D5B8}','\u{1D5EC}','\u{1D620}','\u{1D654}','\u{1D688}','\u{03A5}','\u{03D2}','\u{1D6BC}','\u{1D6F6}','\u{1D730}','\u{1D76A}','\u{1D7A4}','\u{2CA8}','\u{04AE}','\u{13A9}','\u{13BD}','\u{A4EC}','\u{118A4}','\u{102B2}','\u{00A5}','\u{024E}','\u{04B0}']);

    confusables.insert('Z', vec!['\u{102F5}','\u{118E5}','\u{FF3A}','\u{2124}','\u{2128}','\u{1D419}','\u{1D44D}','\u{1D481}','\u{1D4B5}','\u{1D4E9}','\u{1D585}','\u{1D5B9}','\u{1D5ED}','\u{1D621}','\u{1D655}','\u{1D689}','\u{0396}','\u{1D6AD}','\u{1D6E7}','\u{1D721}','\u{1D75B}','\u{1D795}','\u{13C3}','\u{A4DC}','\u{118A9}','\u{01B5}','\u{0224}']);

    confusables.insert('a', vec!['\u{237a}','\u{FF41}','\u{1D41A}','\u{1D44E}','\u{1D482}','\u{1D4B6}','\u{1D4EA}','\u{1D51E}','\u{1D552}','\u{1D586}','\u{1D5BA}','\u{1D5EE}','\u{1D622}','\u{1D656}','\u{1D68A}','\u{0251}','\u{03B1}','\u{1D6C2}','\u{1D6FC}','\u{1D736}','\u{1D770}','\u{1D7AA}','\u{0430}','\u{2376}','\u{0103}','\u{01CE}','\u{0227}','\u{00E5}','\u{1E9A}','\u{1EA3}']);
/*
    confusables.insert('b', vec!['\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}',]);

    confusables.insert('c', vec!['\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}',]);

    confusables.insert('d', vec!['\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}',]);

    confusables.insert('e', vec!['\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}',]);

    confusables.insert('f', vec!['\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}',]);

    confusables.insert('g', vec!['\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}',]);

    confusables.insert('h', vec!['\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}',]);

    confusables.insert('i', vec!['\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}',]);

    confusables.insert('j', vec!['\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}',]);

    confusables.insert('k', vec!['\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}',]);

    confusables.insert('l', vec!['\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}',]);

    confusables.insert('m', vec!['\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}',]);

    confusables.insert('n', vec!['\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}',]);

    confusables.insert('o', vec!['\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}',]);

    confusables.insert('p', vec!['\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}',]);

    confusables.insert('q', vec!['\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}',]);

    confusables.insert('r', vec!['\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}',]);

    confusables.insert('s', vec!['\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}',]);

    confusables.insert('t', vec!['\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}',]);

    confusables.insert('u', vec!['\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}',]);

    confusables.insert('v', vec!['\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}',]);

    confusables.insert('w', vec!['\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}',]);

    confusables.insert('x', vec!['\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}',]);

    confusables.insert('y', vec!['\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}',]);

    confusables.insert('z', vec!['\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}','\u{}',]);
*/

    let mut output = String::new();
    let mut input_chars = input.chars().peekable();
    while input_chars.peek() != None {
        let next = input_chars.next().unwrap();
        //don't process new lines and spaces, but allow them in inputs
        if next == '\n' || next == ' ' {
            output.push(next);
            continue;
        }
        //TODO: handle the error cases here instead of just blindly unwrapping.
        let next_out = rand::thread_rng().choose(confusables.get(&next).unwrap());
        output.push(*next_out.unwrap());
    }
    return output;
}