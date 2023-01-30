#[cfg(test)]
mod script_template_tests {
    use bsv::{MatchDataTypes, Script, ScriptTemplate};

    #[test]
    fn empty_script_does_not_match_template() {
        let script = Script::default();

        let script_template = ScriptTemplate::from_asm_string(
            "d26f2b12ee0a5923dab7314e533917f2ab5b50da5ce302d3d60941f0ee8000a2 21e8 OP_SIZE OP_4 OP_PICK OP_SHA256 OP_SWAP OP_SPLIT OP_DROP OP_EQUALVERIFY OP_DROP OP_CHECKSIG",
        )
        .unwrap();

        assert_eq!(script.is_match(&script_template), false);
    }

    #[test]
    fn exact_script_template_matches_script_without_extracting_data() {
        let script =
            Script::from_asm_string("d26f2b12ee0a5923dab7314e533917f2ab5b50da5ce302d3d60941f0ee8000a2 21e8 OP_SIZE OP_4 OP_PICK OP_SHA256 OP_SWAP OP_SPLIT OP_DROP OP_EQUALVERIFY OP_DROP OP_CHECKSIG")
                .unwrap();

        println!("Script Test {:?}", script);
        let script_template = ScriptTemplate::from_asm_string(
            "d26f2b12ee0a5923dab7314e533917f2ab5b50da5ce302d3d60941f0ee8000a2 21e8 OP_SIZE OP_4 OP_PICK OP_SHA256 OP_SWAP OP_SPLIT OP_DROP OP_EQUALVERIFY OP_DROP OP_CHECKSIG",
        )
        .unwrap();

        let match_result = script.matches(&script_template);

        println!("Matches? {:?}", match_result);
        assert_eq!(match_result.is_ok(), true);

        let extracted = match_result.unwrap();

        assert!(extracted.is_empty());
    }

    #[test]
    fn exact_script_template_matches_script_without_extracting_data_should_fail() {
        let script =
            Script::from_asm_string("d26f2b12ee0a5923dab7314e533917f2ab5b50da5ce302d3d60941f0ee8000a2 21e8 OP_SIZE OP_4 OP_PICK OP_SHA256 OP_SWAP OP_SPLIT OP_DROP OP_EQUALVERIFY OP_DROP OP_CHECKSIG")
                .unwrap();

        println!("Script Test {:?}", script);
        let script_template = ScriptTemplate::from_asm_string(
            "3333333333333333333333333333333333333333333333333333333333333333 21e8 OP_SIZE OP_4 OP_PICK OP_SHA256 OP_SWAP OP_SPLIT OP_DROP OP_EQUALVERIFY OP_DROP OP_CHECKSIG",
        )
        .unwrap();

        let match_result = script.matches(&script_template);

        println!("Matches? {:?}", match_result);
        assert_eq!(match_result.is_err(), true);
    }

    #[test]
    fn op_pubkeyhash_matches_p2pkh_script_template() {
        let script = Script::from_asm_string("OP_DUP OP_HASH160 05186ff0711831d110ca96ddfc47816b5a31900d OP_EQUALVERIFY OP_CHECKSIG").unwrap();

        let script_template = ScriptTemplate::from_asm_string("OP_DUP OP_HASH160 OP_PUBKEYHASH OP_EQUALVERIFY OP_CHECKSIG").unwrap();

        let match_result = script.matches(&script_template);
        assert_eq!(match_result.is_ok(), true);

        let extracted = match_result.unwrap();
        assert_eq!(extracted.len(), 1);

        match &extracted[0] {
            (MatchDataTypes::PublicKeyHash, v) => assert_eq!(v, &hex::decode("05186ff0711831d110ca96ddfc47816b5a31900d").unwrap()),
            _ => assert!(false, "Index 0 did not contain a PubKeyHash"),
        }
    }

    #[test]
    fn op_data_script_template_matches_21e8_puzzle() {
        let script =
            Script::from_asm_string("d26f2b12ee0a5923dab7314e533917f2ab5b50da5ce302d3d60941f0ee8000a2 21e8 OP_SIZE OP_4 OP_PICK OP_SHA256 OP_SWAP OP_SPLIT OP_DROP OP_EQUALVERIFY OP_DROP OP_CHECKSIG")
                .unwrap();

        let script_template = ScriptTemplate::from_asm_string("OP_DATA=32 21e8 OP_SIZE OP_4 OP_PICK OP_SHA256 OP_SWAP OP_SPLIT OP_DROP OP_EQUALVERIFY OP_DROP OP_CHECKSIG").unwrap();
        let match_result = script.matches(&script_template);
        assert_eq!(match_result.is_ok(), true);

        let extracted = match_result.unwrap();
        assert_eq!(extracted.len(), 1);

        match &extracted[0] {
            (MatchDataTypes::Data, v) => {
                assert_eq!(v.len(), 32, "Data was not 32 bytes long");
                assert_eq!(v, &hex::decode("d26f2b12ee0a5923dab7314e533917f2ab5b50da5ce302d3d60941f0ee8000a2").unwrap())
            }
            _ => assert!(false, "Index 0 did not contain Data"),
        }
    }

    #[test]
    fn op_data_script_template_matches_hash_puzzle() {
        let script =
            Script::from_asm_string("d26f2b12ee0a5923dab7314e533917f2ab5b50da5ce302d3d60941f0ee8000a2 21e8 OP_SIZE OP_4 OP_PICK OP_SHA256 OP_SWAP OP_SPLIT OP_DROP OP_EQUALVERIFY OP_DROP OP_CHECKSIG")
                .unwrap();

        let script_template = ScriptTemplate::from_asm_string("OP_DATA=32 OP_DATA=2 OP_SIZE OP_4 OP_PICK OP_SHA256 OP_SWAP OP_SPLIT OP_DROP OP_EQUALVERIFY OP_DROP OP_CHECKSIG").unwrap();
        let match_result = script.matches(&script_template);
        assert_eq!(match_result.is_ok(), true);

        let extracted = match_result.unwrap();
        assert_eq!(extracted.len(), 2);

        match &extracted[0] {
            (MatchDataTypes::Data, v) => {
                assert_eq!(v.len(), 32, "Data was not 32 bytes long");
                assert_eq!(v, &hex::decode("d26f2b12ee0a5923dab7314e533917f2ab5b50da5ce302d3d60941f0ee8000a2").unwrap())
            }
            _ => assert!(false, "Index 0 did not contain Data"),
        }

        match &extracted[1] {
            (MatchDataTypes::Data, v) => {
                assert_eq!(v.len(), 2, "Data was not 2 bytes long");
                assert_eq!(v, &hex::decode("21e8").unwrap())
            }
            _ => assert!(false, "Index 1 did not contain Data"),
        }
    }

    #[test]
    fn p2pkh_script_template_doesnt_match_21e8_puzzle() {
        let script =
            Script::from_asm_string("d26f2b12ee0a5923dab7314e533917f2ab5b50da5ce302d3d60941f0ee8000a2 21e8 OP_SIZE OP_4 OP_PICK OP_SHA256 OP_SWAP OP_SPLIT OP_DROP OP_EQUALVERIFY OP_DROP OP_CHECKSIG")
                .unwrap();

        let script_template = ScriptTemplate::from_asm_string("OP_DUP OP_HASH160 OP_PUBKEYHASH OP_EQUALVERIFY OP_CHECKSIG").unwrap();

        assert!(script.matches(&script_template).is_err());
    }

    //     #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    //     fn p2pkh_script_template_doesnt_match_21e8_puzzle_wasm() {
    //         let script =
    //             Script::from_asm_string("d26f2b12ee0a5923dab7314e533917f2ab5b50da5ce302d3d60941f0ee8000a2 21e8 OP_SIZE OP_4 OP_PICK OP_SHA256 OP_SWAP OP_SPLIT OP_DROP OP_EQUALVERIFY OP_DROP OP_CHECKSIG")
    //                 .unwrap();

    //         let script_template = ScriptTemplate::from_asm_string("OP_DUP OP_HASH160 OP_PUBKEYHASH OP_EQUALVERIFY OP_CHECKSIG").unwrap();

    //         assert_eq!(script.is_match(&script_template), false)
    //     }

    #[test]
    fn pubkey_script_template_matches_compressed_p2pk() {
        let script = Script::from_asm_string("03652aee2d0a773eccc4bc5f7816bd4c525f408da26422171a22829bfde4109296 OP_CHECKSIG").unwrap();

        let script_template = ScriptTemplate::from_asm_string("OP_PUBKEY OP_CHECKSIG").unwrap();

        let match_result = script.matches(&script_template);
        assert_eq!(match_result.is_ok(), true);

        let extracted = match_result.unwrap();
        assert_eq!(extracted.len(), 1);

        match &extracted[0] {
            (MatchDataTypes::PublicKey, v) => {
                assert_eq!(v.len(), 33, "Data was not 32 bytes long");
                assert_eq!(v, &hex::decode("03652aee2d0a773eccc4bc5f7816bd4c525f408da26422171a22829bfde4109296").unwrap())
            }
            _ => assert!(false, "Index 0 did not contain Data"),
        }
    }

    #[test]
    fn pubkey_script_template_matches_uncompressed_p2pk() {
        let script = Script::from_asm_string("04652aee2d0a773eccc4bc5f7816bd4c525f408da26422171a22829bfde41092967872982dc75cb2d4b5162f17e97cdcf4329e9fa4ef1b3cba155ccdb600d199b1 OP_CHECKSIG").unwrap();

        let script_template = ScriptTemplate::from_asm_string("OP_PUBKEY OP_CHECKSIG").unwrap();

        let match_result = script.matches(&script_template);
        assert_eq!(match_result.is_ok(), true);

        let extracted = match_result.unwrap();
        assert_eq!(extracted.len(), 1);

        match &extracted[0] {
            (MatchDataTypes::PublicKey, v) => {
                assert_eq!(v.len(), 65, "Data was not 65 bytes long");
                assert_eq!(
                    v,
                    &hex::decode("04652aee2d0a773eccc4bc5f7816bd4c525f408da26422171a22829bfde41092967872982dc75cb2d4b5162f17e97cdcf4329e9fa4ef1b3cba155ccdb600d199b1").unwrap()
                )
            }
            _ => assert!(false, "Index 0 did not contain PubKey"),
        }
    }

    #[test]
    fn multi_pub_key_and_p2pkh_script_template() {
        let script = Script::from_asm_string("04652aee2d0a773eccc4bc5f7816bd4c525f408da26422171a22829bfde41092967872982dc75cb2d4b5162f17e97cdcf4329e9fa4ef1b3cba155ccdb600d199b1 OP_CHECKSIG OP_1 OP_DUP 03652aee2d0a773eccc4bc5f7816bd4c525f408da26422171a22829bfde4109296 OP_CHECKSIG OP_DUP OP_HASH160 05186ff0711831d110ca96ddfc47816b5a31900d OP_EQUALVERIFY OP_CHECKSIG").unwrap();

        let script_template = ScriptTemplate::from_asm_string("OP_PUBKEY OP_CHECKSIG OP_1 OP_DUP OP_PUBKEY OP_CHECKSIG OP_DUP OP_HASH160 OP_PUBKEYHASH OP_EQUALVERIFY OP_CHECKSIG").unwrap();

        let match_result = script.matches(&script_template);
        assert_eq!(match_result.is_ok(), true);

        let extracted = match_result.unwrap();
        assert_eq!(extracted.len(), 3);

        match &extracted[0] {
            (MatchDataTypes::PublicKey, v) => {
                assert_eq!(v.len(), 65, "Data was not 65 bytes long");
                assert_eq!(
                    v,
                    &hex::decode("04652aee2d0a773eccc4bc5f7816bd4c525f408da26422171a22829bfde41092967872982dc75cb2d4b5162f17e97cdcf4329e9fa4ef1b3cba155ccdb600d199b1").unwrap()
                )
            }
            _ => assert!(false, "Index 0 did not contain PubKey"),
        }

        match &extracted[1] {
            (MatchDataTypes::PublicKey, v) => {
                assert_eq!(v.len(), 33, "Data was not 33 bytes long");
                assert_eq!(v, &hex::decode("03652aee2d0a773eccc4bc5f7816bd4c525f408da26422171a22829bfde4109296").unwrap())
            }
            _ => assert!(false, "Index 1 did not contain PubKey"),
        }

        match &extracted[2] {
            (MatchDataTypes::PublicKeyHash, v) => {
                assert_eq!(v.len(), 20, "Data was not 20 bytes long");
                assert_eq!(v, &hex::decode("05186ff0711831d110ca96ddfc47816b5a31900d").unwrap())
            }
            _ => assert!(false, "Index 1 did not contain PubKeyHash"),
        }
    }

    #[test]
    fn fully_formed_p2pkh_matches_with_script_template() {
        let script = Script::from_asm_string("304402206173a490a5e62036e64f77f8c98db6c57f162a68147cb276bc61da589a114e27022053c19c60dbe7a97ce609631071ee5293c6e6bf4b859094c25a3385490f772c5541 0319a38fb498ff221b6e1b528b911c62f6ff2ac5023405c637859e4d7ff28f265d OP_DUP OP_HASH160 08ed73ac2a3564dd1a431c61f7c2ce6b64e1fe80 OP_EQUALVERIFY OP_CHECKSIG").unwrap();

        let script_template = ScriptTemplate::from_asm_string("OP_SIG OP_PUBKEY OP_DUP OP_HASH160 OP_PUBKEYHASH OP_EQUALVERIFY OP_CHECKSIG").unwrap();

        let match_result = script.matches(&script_template);
        assert_eq!(match_result.is_ok(), true, "Failed to match script");

        let extracted = match_result.unwrap();
        assert_eq!(extracted.len(), 3);

        match &extracted[0] {
            (MatchDataTypes::Signature, v) => {
                assert_eq!(v.len(), 71, "Signature was not 71 bytes long");
                assert_eq!(
                    v,
                    &hex::decode("304402206173a490a5e62036e64f77f8c98db6c57f162a68147cb276bc61da589a114e27022053c19c60dbe7a97ce609631071ee5293c6e6bf4b859094c25a3385490f772c5541").unwrap()
                )
            }
            _ => assert!(false, "Index 0 did not contain Signature"),
        }

        match &extracted[1] {
            (MatchDataTypes::PublicKey, v) => {
                assert_eq!(v.len(), 33, "PubKey was not 33 bytes long");
                assert_eq!(v, &hex::decode("0319a38fb498ff221b6e1b528b911c62f6ff2ac5023405c637859e4d7ff28f265d").unwrap())
            }
            _ => assert!(false, "Index 1 did not contain PublicKey"),
        }

        match &extracted[2] {
            (MatchDataTypes::PublicKeyHash, v) => {
                assert_eq!(v.len(), 20, "PubKeyHash was not 20 bytes long");
                assert_eq!(v, &hex::decode("08ed73ac2a3564dd1a431c61f7c2ce6b64e1fe80").unwrap())
            }
            _ => assert!(false, "Index 2 did not contain PublicKeyHash"),
        }
    }

    #[test]
    fn matches_nft() {
        let script = Script::from_asm_string("OP_HASH160 b8bcb07f6344b42ab04250c86a6e8b75d3fdbbc6 OP_EQUALVERIFY OP_DUP OP_HASH160 f9dfc5a4ae5256e5938c2d819738f7b57e4d7b46 OP_EQUALVERIFY OP_CHECKSIG OP_RETURN 7b227469746c65223a22547572626f20466f78202331222c226465736372697074696f6e223a225765206c696b652074686520666f78222c226e756d626572223a312c22736572696573223a36392c22696d616765223a22623a2f2f33376136636339636639613461613662356632316534333331363935666666613466323039363335366239633636336436393636333962336363303765376531222c2261747472696275746573223a5b7b2274726169745f74797065223a22436f6c6f72222c2276616c7565223a224f72616e6765227d2c7b2274726169745f74797065223a22446975726e616c697479222c2276616c7565223a22446179227d5d7d").unwrap();

        let script_template = ScriptTemplate::from_asm_string("OP_HASH160 OP_DATA=20 OP_EQUALVERIFY OP_DUP OP_HASH160 OP_PUBKEYHASH OP_EQUALVERIFY OP_CHECKSIG OP_RETURN OP_DATA").unwrap();

        let match_result = script.matches(&script_template);
        assert_eq!(match_result.is_ok(), true);

        let extracted = match_result.unwrap();
        assert_eq!(extracted.len(), 3);

        match &extracted[0] {
            (MatchDataTypes::Data, v) => {
                assert_eq!(v.len(), 20, "Data was not 20 bytes long");
                assert_eq!(v, &hex::decode("b8bcb07f6344b42ab04250c86a6e8b75d3fdbbc6").unwrap())
            }
            _ => assert!(false, "Index 0 did not contain Signature"),
        }

        match &extracted[1] {
            (MatchDataTypes::PublicKeyHash, v) => {
                assert_eq!(v.len(), 20, "PubKeyhash was not 20 bytes long");
                assert_eq!(v, &hex::decode("f9dfc5a4ae5256e5938c2d819738f7b57e4d7b46").unwrap())
            }
            _ => assert!(false, "Index 1 did not contain PubKeyhash"),
        }

        match &extracted[2] {
            (MatchDataTypes::Data, v) => {
                assert_eq!(v, &hex::decode("7b227469746c65223a22547572626f20466f78202331222c226465736372697074696f6e223a225765206c696b652074686520666f78222c226e756d626572223a312c22736572696573223a36392c22696d616765223a22623a2f2f33376136636339636639613461613662356632316534333331363935666666613466323039363335366239633636336436393636333962336363303765376531222c2261747472696275746573223a5b7b2274726169745f74797065223a22436f6c6f72222c2276616c7565223a224f72616e6765227d2c7b2274726169745f74797065223a22446975726e616c697479222c2276616c7565223a22446179227d5d7d").unwrap())
            }
            _ => assert!(false, "Index 2 did not contain Data"),
        }
    }

    #[test]
    fn matches_sigil_purchase_tx() {
        use bsv::{MatchCriteria, Transaction};

        let tx = Transaction::from_compact_hex("A46776657273696F6E0266696E7075747382A66A707265765F74785F696478406661623130323139393064303535653933336163343463323463363436333064626639343532323265323138346335356566393663383866663635323562653164766F757418436A7363726970745F73696783788E3330343430323230356538616231313038336336303235343264616633343831633937373564386462303463666566313735643839616530663261383732663833653461643463343032323037643337663436313637316430373466373362333834393436323164616235363162356533346638383038633962653432386238333032396533303831326332633378423032316338353931633566323034633134363332363066643430343466303261656638643761356461616662373661633662656635313132633263643136396131637840303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030306873657175656E63651AFFFFFFFF70756E6C6F636B696E675F7363726970748A6A4F505F484153483136307828623862636230376636333434623432616230343235306338366136653862373564336664626263366E4F505F455155414C564552494659664F505F4455506A4F505F484153483136307828663964666335613461653532353665353933386332643831393733386637623537653464376234366E4F505F455155414C5645524946596B4F505F434845434B534947694F505F52455455524E826C4F505F5055534844415441317901FC37623232373436393734366336353232336132323534373537323632366632303436366637383230323333363339323232633232363436353733363337323639373037343639366636653232336132323537363532303663363936623635323037343638363532303636366637383232326332323665373536643632363537323232336133363339326332323733363537323639363537333232336133363339326332323639366436313637363532323361323236323361326632663333333736313336363336333339363336363339363133343631363133363632333536363332333136353334333333333331333633393335363636363636363133343636333233303339333633333335333636323339363333363336333336343336333933363336333333393632333336333633333033373635333736353331323232633232363137343734373236393632373537343635373332323361356237623232373437323631363937343566373437393730363532323361323234333666366336663732323232633232373636313663373536353232336132323532363536343232376432633762323237343732363136393734356637343739373036353232336132323434363937353732366536313663363937343739323232633232373636313663373536353232336132323434363137393232376435643764687361746F7368697318DAA46A707265765F74785F696478406565633364383266303162613766363833623863633263313632656238393936373464333836383237353734636162383230656364663464333139333135666264766F7574016A7363726970745F73696782789033303435303232313030633963633537383039663631393565656665366261306161626437666436313165616138333463656165343532646332373130303961633634316262373130623032323033633531383564646436626266613066313233373638356237343035373731306531623834346132633364393163623435393965353538646662386338303231343178423033366433396130373439636137383030623531623339663763613764313037386237666338373035396534376165653431363966313563393936316236353838646873657175656E63651AFFFFFFFF676F75747075747382A26576616C756518DA6E7363726970745F7075625F6B65798A6A4F505F484153483136307828623862636230376636333434623432616230343235306338366136653862373564336664626263366E4F505F455155414C564552494659664F505F4455506A4F505F484153483136307828313461383033366338623364393130613765323464343630363730343864383736313237346235356E4F505F455155414C5645524946596B4F505F434845434B534947694F505F52455455524E826C4F505F5055534844415441317901FC37623232373436393734366336353232336132323534373537323632366632303436366637383230323333363339323232633232363436353733363337323639373037343639366636653232336132323537363532303663363936623635323037343638363532303636366637383232326332323665373536643632363537323232336133363339326332323733363537323639363537333232336133363339326332323639366436313637363532323361323236323361326632663333333736313336363336333339363336363339363133343631363133363632333536363332333136353334333333333331333633393335363636363636363133343636333233303339333633333335333636323339363333363336333336343336333933363336333333393632333336333633333033373635333736353331323232633232363137343734373236393632373537343635373332323361356237623232373437323631363937343566373437393730363532323361323234333666366336663732323232633232373636313663373536353232336132323532363536343232376432633762323237343732363136393734356637343739373036353232336132323434363937353732366536313663363937343739323232633232373636313663373536353232336132323434363137393232376435643764A26576616C75651A000DE1066E7363726970745F7075625F6B657985664F505F4455506A4F505F484153483136307828333934613034363761313739636130303237656530616638623565346530666165666637316462616E4F505F455155414C5645524946596B4F505F434845434B5349476A6E5F6C6F636B74696D6500").unwrap();

        let tmp = ScriptTemplate::from_asm_string("OP_SIG OP_PUBKEY OP_DATA OP_HASH160 OP_DATA OP_EQUALVERIFY OP_DUP OP_HASH160 OP_PUBKEYHASH OP_EQUALVERIFY OP_CHECKSIG OP_RETURN OP_DATA").unwrap();
        let final_match = tx.get_input(0).unwrap().get_finalised_script().unwrap().matches(&tmp).unwrap();

        assert_eq!(final_match.is_empty(), false);
        assert_eq!(tx.get_input(1).unwrap().get_finalised_script().unwrap().matches(&tmp).is_err(), true);

        let criteria = MatchCriteria::new().set_script_template(&tmp);

        let matching_inputs = tx.match_inputs(&criteria);

        assert_eq!(matching_inputs.len(), 1);
    }

    #[test]
    fn matches_turbo_fox_transfer_tx() {
        use bsv::{MatchCriteria, Transaction};

        let tx = Transaction::from_compact_hex("a46776657273696f6e0266696e7075747388a66a707265765f74785f696478403462316666393533366361643030303364356661356162363133303366366639656563646532306665613166333461326164366136386235626339653735353264766f7574006a7363726970745f73696783789033303435303232313030643665326464393138376238393136613439656535633063623637373231626162373865386662343036303033616361376238366435313238303635636533313032323031393731353331396235656162333563353735643735663339386439643433306337386636626332643438663430323834356530316236623361633061613831633378423032376237366661653665643030653537323366653262306539383762646363623334656566633132393333373631366434613335653964653464306131303035307840303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030306873657175656e63651affffffff70756e6c6f636b696e675f7363726970748a6a4f505f484153483136307828623862636230376636333434623432616230343235306338366136653862373564336664626263366e4f505f455155414c564552494659664f505f4455506a4f505f484153483136307828303063363730313565653534373566326436663366363934663331333061663036346334393035356e4f505f455155414c5645524946596b4f505f434845434b534947694f505f52455455524e826c4f505f5055534844415441327902063762323237343639373436633635323233613232353437353732363236663230343636663738323032333335333632323263323236343635373336333732363937303734363936663665323233613232353736353230366336393662363532303734363836353230363636663738323232633232366537353664363236353732323233613335333632633232373336353732363936353733323233613336333932633232363936643631363736353232336132323632336132663266333333373631333636333633333936333636333936313334363136313336363233353636333233313635333433333333333133363339333536363636363636313334363633323330333933363333333533363632333936333336333633333634333633393336333633333339363233333633363333303337363533373635333132323263323236313734373437323639363237353734363537333232336135623762323237343732363136393734356637343739373036353232336132323433366636633666373232323263323237363631366337353635323233613232346637323631366536373635323237643263376232323734373236313639373435663734373937303635323233613232343436393735373236653631366336393734373932323263323237363631366337353635323233613232346536393637363837343232376435643764687361746f7368697318daa56a707265765f74785f696478406437383838356338393934363933313833373664313834373630383630383561356434393464633732336633623336383564363039353666666638616335343064766f7574056a7363726970745f73696782788e3330343430323230363635343861396333343430396666643738646164356534336330333833663465643133663365313562333231323933393664646365633664663064393965613032323034333932653334616138353866333836663966346133303166346331363636343838326137663732333462323563333233363164613165393733626263633564343178423033373130643937386637346632353662663162383233356165623663393863303066373364643632393066646261353032633139333331633638616232666435386873657175656e63651affffffff70756e6c6f636b696e675f73637269707485664f505f4455506a4f505f484153483136307828316434333936353138623435366137656336323237393866396232633337656361643962343130376e4f505f455155414c5645524946596b4f505f434845434b534947a56a707265765f74785f696478403965643931376233306536363361663166343830383665366362656331623663636564623133396434343334303038353366386162333963633130633562303464766f7574056a7363726970745f73696782788e3330343430323230313733396666323166393836336530363535306339333565663165653163663931393030333230663530633831343739306165336265663733333132376337383032323036336631653163373161356334393465316534373765396462633832323735303336623439653762376238386136633961633061393239613763346263393863343178423033633961633634306139613830613561333665656234396530633133613430343532626535383761303961346264313365386433333731306131613931363533366873657175656e63651affffffff70756e6c6f636b696e675f73637269707485664f505f4455506a4f505f484153483136307828626162346533343537613564396530346462656662633534623763373236666662623130383039336e4f505f455155414c5645524946596b4f505f434845434b534947a56a707265765f74785f696478406132323061396631313839616234653662616437336335383434666536333534633064366464396238383038336636623731383362653364343037376165643264766f7574056a7363726970745f73696782788e3330343430323230333432383861313761336563636134366337316436373438303330313635363634633862343336623436373333616165363962343761643038353031623834363032323032366239363462303236393364663033336235623462646663393666313262633661303466393565343831356135336430323631396164346139313466343230343178423032633830393333353563616466633932633261393565383335616235633437316430646439383966316638383838333764633664383364306164353937643938356873657175656e63651affffffff70756e6c6f636b696e675f73637269707485664f505f4455506a4f505f484153483136307828336336393030353534333037653935636539363764306534303537626530346533636563363064376e4f505f455155414c5645524946596b4f505f434845434b534947a56a707265765f74785f696478406637326137633339396665373264616466373539323835386539313163336362656163313434643566393331303931336463663431303562613937386336626564766f7574046a7363726970745f73696782789033303435303232313030626465333839613037613865376238633236646163323761373662306166626562346265333265653031316332306537386164636666396461613731373561363032323034383730363165663261396364663465613263626661306432313332303762353165306363363134386561343837666363356439323464306536363239346138343178423033653965626663393738303034356234313337623135386233383163333265323538393465666166303936343036336435663565663231343735653133613238376873657175656e63651affffffff70756e6c6f636b696e675f73637269707485664f505f4455506a4f505f484153483136307828623263383431373935363163393536663538316235323966646635316137356466323361616632616e4f505f455155414c5645524946596b4f505f434845434b534947a56a707265765f74785f696478403134643163343837373835656265666364316638333035623631663366666433643464616563326339646635643361323536366533363933323161396566343864766f7574046a7363726970745f73696782788e3330343430323230326462313635653030323433633234346638643565303864616334333131393537363734623063353266656630663534306165356437666334363132363635343032323031313265633230383963363163366561323430643765373264613762613031396163613564393662313930323132643236643939396439663361313566663530343178423033613166336362376538376530306665363439633739353634343363393163326238646332303337646437643566613663313839333236396234303561373166646873657175656e63651affffffff70756e6c6f636b696e675f73637269707485664f505f4455506a4f505f484153483136307828373238613134616462323730306330616330643239653134303365633534343364666264316534316e4f505f455155414c5645524946596b4f505f434845434b534947a56a707265765f74785f696478406636353266303235663965353764353834376164633965376437383239336566376265623832373664323533616338623737373831326530666438653136623264766f7574056a7363726970745f73696782788e3330343430323230363637623964333231376335316633326538636635643135323164666335386539336435373030656162356236646232636333653261343831316238616131333032323035653963633634656263626439666235656431663231663062303464373164333963303331643436663039396363386138376261616439353865646130333630343178423033376461333038646164653134373431383237313365323961383663323762643233383332633739326531663363613937623735616534343437663932326561626873657175656e63651affffffff70756e6c6f636b696e675f73637269707485664f505f4455506a4f505f484153483136307828656234373165303661383063336364653537643138323337363865623934653732366364306162366e4f505f455155414c5645524946596b4f505f434845434b534947a56a707265765f74785f696478403238386130663335316239373865303532303165613037353264363834353966623264383430646565633735396333643164336135333266393333626439663364766f7574056a7363726970745f73696782788e3330343430323230373831633338306139323933656438313563613239343261383265666537383463323364646637656163626565376234613239376435633933613163313736303032323037323731346438336434353466366666316237383634383662393465306135613634646530333963623036303238343763383065356439396232343630333861343178423032656636393332373638633835653565323161666334343833663031303932613336326662666231633066363966353036306366656334323963633531636434336873657175656e63651affffffff70756e6c6f636b696e675f73637269707485664f505f4455506a4f505f484153483136307828346435306234663331646264613165303061373332343466656434613161373432383336303866636e4f505f455155414c5645524946596b4f505f434845434b534947676f75747075747382a26576616c756518da6e7363726970745f7075625f6b65798a6a4f505f484153483136307828623862636230376636333434623432616230343235306338366136653862373564336664626263366e4f505f455155414c564552494659664f505f4455506a4f505f484153483136307828303063363730313565653534373566326436663366363934663331333061663036346334393035356e4f505f455155414c5645524946596b4f505f434845434b534947694f505f52455455524e826c4f505f5055534844415441327902063762323237343639373436633635323233613232353437353732363236663230343636663738323032333335333632323263323236343635373336333732363937303734363936663665323233613232353736353230366336393662363532303734363836353230363636663738323232633232366537353664363236353732323233613335333632633232373336353732363936353733323233613336333932633232363936643631363736353232336132323632336132663266333333373631333636333633333936333636333936313334363136313336363233353636333233313635333433333333333133363339333536363636363636313334363633323330333933363333333533363632333936333336333633333634333633393336333633333339363233333633363333303337363533373635333132323263323236313734373437323639363237353734363537333232336135623762323237343732363136393734356637343739373036353232336132323433366636633666373232323263323237363631366337353635323233613232346637323631366536373635323237643263376232323734373236313639373435663734373937303635323233613232343436393735373236653631366336393734373932323263323237363631366337353635323233613232346536393637363837343232376435643764a26576616c75651a0001d4796e7363726970745f7075625f6b657985664f505f4455506a4f505f484153483136307828386435666431653862643637353539646435393938623166633039326562663736643239396631656e4f505f455155414c5645524946596b4f505f434845434b5349476a6e5f6c6f636b74696d6500").unwrap();

        let tmp2 = ScriptTemplate::from_asm_string("OP_SIG OP_PUBKEY 0000000000000000000000000000000000000000000000000000000000000000 OP_HASH160 b8bcb07f6344b42ab04250c86a6e8b75d3fdbbc6 OP_EQUALVERIFY OP_DUP OP_HASH160 OP_PUBKEYHASH OP_EQUALVERIFY OP_CHECKSIG OP_RETURN OP_DATA").unwrap();
        println!("{:?}", tmp2);

        let tmp = ScriptTemplate::from_asm_string("OP_SIG OP_PUBKEY 0000000000000000000000000000000000000000000000000000000000000000 OP_HASH160 b8bcb07f6344b42ab04250c86a6e8b75d3fdbbc6 OP_EQUALVERIFY OP_DUP OP_HASH160 OP_PUBKEYHASH OP_EQUALVERIFY OP_CHECKSIG OP_RETURN OP_DATA").unwrap();
        let final_match = tx.get_input(0).unwrap().get_finalised_script().unwrap().matches(&tmp).unwrap();

        assert_eq!(final_match.is_empty(), false);
        assert_eq!(tx.get_input(1).unwrap().get_finalised_script().unwrap().matches(&tmp).is_err(), true);

        let criteria = MatchCriteria::new().set_script_template(&tmp);

        let matching_inputs = tx.match_inputs(&criteria);

        assert_eq!(matching_inputs.len(), 1);
    }
}
