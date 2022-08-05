//! raydium v1 vault account address, used to derive all other addresses

use anchor_lang::solana_program::{self, pubkey::Pubkey};
use static_pubkey::static_pubkey;

/// vault address account for the RAY_USDT farm
pub const RAY_USDT: Pubkey = static_pubkey!("1ZpdBUTiDLTUe3izSdYfRXSf93fpJPmoKtA5bFjGesS");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const RAY_USDT_INFO_ACCOUNT: Pubkey =
    static_pubkey!("Bo8xiDzWWJgGgjG1YMriLT6hNkQX7NzeeemjkmXiwJmp");
/// vault lp token account for the RAY_USDT farm
pub const RAY_USDT_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("GoW9fQtCrX1Bd4abWzETcpewbeCyQQWEWCyWmnQg3coK");

/// vault address account for the RAY_USDC farm
pub const RAY_USDC: Pubkey = static_pubkey!("HvNpbHuQUqGG748ZzgzcH5216wdQdTc283CEyFMc3RdG");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const RAY_USDC_INFO_ACCOUNT: Pubkey =
    static_pubkey!("Gf38RxSF3FguiBVYfsB8AmpPyNkGCrNDE7LNvr6U8n7C");
/// vault lp token account for the RAY_USDC farm
pub const RAY_USDC_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("E8gJAEcHDB4be9sCKSytLUyBe3V5SEDHgn4192REJhaB");
/// old vault info account for the RAY_USDC farm, this is the output of the 'derive_vault_user_info_address' function
pub const RAY_USDC_OLD_INFO: Pubkey =
    static_pubkey!("8vnMSWpzW2RVdAeMaqXKGbQ3r11ijf6vrCm28Ks1bXRA");

/// vault address account for the RAY_SRM farm
pub const RAY_SRM: Pubkey = static_pubkey!("EkePqacuxaubJJxCYW9RxqyXc1r4LzLJTRfF4bW64UQv");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const RAY_SRM_INFO_ACCOUNT: Pubkey =
    static_pubkey!("8RGsvBeGytCzpf8jfFSbLygvSSLsb4tPgqqQqvyrXfKx");
/// vault lp token account for the RAY_SRM farm
pub const RAY_SRM_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("9jngyDQiEhYh7RWVS5SCJnyhN3kPjG8rJBJeKcFGs9eB");
/// old vault info account for the RAY_SRM farm, this is the output of the 'derive_vault_user_info_address' function
pub const RAY_SRM_OLD_INFO: Pubkey = static_pubkey!("36ckujxHmZLyqvzfT5wE94tHEgYfMkChPKAhkGUyGkYW");

/// vault address account for the RAY_SOL farm
pub const RAY_SOL: Pubkey = static_pubkey!("HbLCyHdEK2btVvYny87as5xx9ap7RdMdXAMujSE5Ukw1");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const RAY_SOL_INFO_ACCOUNT: Pubkey =
    static_pubkey!("4YgYjNXjdTG16v39AYippLxF3jv7ERUZZNLuqWtYqWh2");
/// vault lp token account for the RAY_SOL farm
pub const RAY_SOL_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("4vMsGjjNvviT84rvhQBui9kvtfgGmpwYtM24e3xRw5V");
/// old vault info account for the RAY_SOL farm, this is the output of the 'derive_vault_user_info_address' function
pub const RAY_SOL_OLD_INFO: Pubkey = static_pubkey!("Hdp4Dk9xXDV5ezofS61Y8Q8iQ6EXU9TMwVSWm5Gk8eYu");

/// vault address account for the RAY_ETH farm
pub const RAY_ETH: Pubkey = static_pubkey!("9LgYiX3nio9NhNTyxysr9Y4auApnmK3tw5XodTnM7ur4");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const RAY_ETH_INFO_ACCOUNT: Pubkey =
    static_pubkey!("CU34bCFZzb1nUENrJwKN7xVLMuQmK8wRMfhHCCWmZnvo");
/// vault lp token account for the RAY_ETH farm
pub const RAY_ETH_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("AvD5tbmKMgVyyDreVTJG5mHmU6qLfSVq8vdmhwq12UNN");
/// old vault info account for the RAY_ETH farm, this is the output of the 'derive_vault_user_info_address' function
pub const RAY_ETH_OLD_INFO: Pubkey = static_pubkey!("7D1uQL18Uy9KRb6dkEmVXYPtLJJPSxxrLgVDXQ9X6aff");

/// vault address account for the STEP_USDC farm
pub const STEP_USDC: Pubkey = static_pubkey!("D7nGJEHb268GLFACtAHz4N42i7P4AuCXaoLBoZd5dyCq");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const STEP_USDC_INFO_ACCOUNT: Pubkey =
    static_pubkey!("FNt5Fmwa6cVJSAWqRbJrommWjfZwjaxuKE6zcVqvvqMB");
/// vault lp token account for the STEP_USDC farm
pub const STEP_USDC_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("B6DsZN93mLrtufvdkzPsMWuDPiSwWp2iSLwAnVVG8bQy");

/// vault address account for the MEDIA_USDC farm
pub const MEDIA_USDC: Pubkey = static_pubkey!("8KHfDLXWxUPqYaqxXMN6K24ViTVPci6LogDrUxu3BXC3");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const MEDIA_USDC_INFO_ACCOUNT: Pubkey =
    static_pubkey!("E3Tt83FDPYNdXHXB6x3tJAAMoYUBn4X9FdxDShbauYV3");
/// vault lp token account for the MEDIA_USDC farm
pub const MEDIA_USDC_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("GhiXnJf2je9KHU6KvyWBAE15yfmKE6vZoUjx1RYiWjvp");

/// vault address account for the KIN_RAY farm
pub const KIN_RAY: Pubkey = static_pubkey!("B2dVgRKsv5ta6VUUDG36yFtV8LKoPSjNzEpnKBiMM8gr");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const KIN_RAY_INFO_ACCOUNT: Pubkey =
    static_pubkey!("CobutwhxH7oCMXGgoiDDw5Fm9Mb1ymZ78sTkTtt6c738");
/// vault lp token account for the KIN_RAY farm
pub const KIN_RAY_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("G8odNj12bsvRfKZ533fszTcaXaofDVXX5SCNXjn4Ki2z");

/// vault address account for the FIDA_RAY farm
pub const FIDA_RAY: Pubkey = static_pubkey!("GZtyUSDgeDVSnrWkJNcM7Duraz5jwE9ok8ZxT55phTXT");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const FIDA_RAY_INFO_ACCOUNT: Pubkey =
    static_pubkey!("23M1rBgZZcKek8aRyH4Ctaf9UybDjkRULEEgvHyFVjEC");
/// vault lp token account for the FIDA_RAY farm
pub const FIDA_RAY_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("8zymriZTyvbpQaqYfh5TD7ShidEeoXwumGbNw49SLdK9");

/// vault address account for the OXY_RAY farm
pub const OXY_RAY: Pubkey = static_pubkey!("H9mcP4Pit5SHajN59UHf1TDv9M2oZT96N87VgnwajzSW");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const OXY_RAY_INFO_ACCOUNT: Pubkey =
    static_pubkey!("3bQ9WogYJ3Me9vK1Dsh3gxAhABfrXRPzrFrpvS31CMtP");
/// vault lp token account for the OXY_RAY farm
pub const OXY_RAY_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("74otQdb7eWERREZVCJ7z297uohZPghFfdGhz51jY8Aas");

/// vault address account for the MAPS_RAY farm
pub const MAPS_RAY: Pubkey = static_pubkey!("47XEvdUVqdevBoyvrtE3wJT2Zn4iDuLCnG4spRQbbLio");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const MAPS_RAY_INFO_ACCOUNT: Pubkey =
    static_pubkey!("EVfW7op7yLrEd69SYSAWx4Q9WacR2V4kbW7MVk4VHQ9F");
/// vault lp token account for the MAPS_RAY farm
pub const MAPS_RAY_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("GiU8GcudGjM5iMn6DsK1cfRRYkRR8uiYpkkePcniXW1N");

/// vault address account for the RAY farm
pub const RAY: Pubkey = static_pubkey!("73RwT8fLVKMEGPYqDsCaf7jbYXWL2vd5AgvSzxpz7e2A");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const RAY_INFO_ACCOUNT: Pubkey = static_pubkey!("7Y15vh9mfrNQmmMnecQ5Bmy1DbDxpeK6EcUfKLhrWiAe");
/// vault lp token account for the RAY farm
pub const RAY_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("FGod8Pt64eCnNmpbbpMC4ysnFa3ufScciQo3gf38pMrt");

/// vault address account for the MER_USDC farm
pub const MER_USDC: Pubkey = static_pubkey!("HjJzWScXQU8NaAByQtCoN9dR6gTAE5uKx5phuMdyLCgW");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const MER_USDC_INFO_ACCOUNT: Pubkey =
    static_pubkey!("Hmz6CM3oRkpgFbdFk8uoZeQUxLuEojuhtSbGJG4Pa4WC");
/// vault lp token account for the MER_USDC farm
pub const MER_USDC_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("CmugE99KxmCKCTHzK1X4E3PH84HnrCbF7qSvRUfYAWae");

/// vault address account for the ROPE_USDC farm
pub const ROPE_USDC: Pubkey = static_pubkey!("Fe1ciKP2en4GiNgEByNgKzraDj6Cin1YLRaWKw5E3L9C");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const ROPE_USDC_INFO_ACCOUNT: Pubkey =
    static_pubkey!("5LmQKpNEn3KmcUUETxANgt3WJjmhvem5ro2tzGctQHFg");
/// vault lp token account for the ROPE_USDC farm
pub const ROPE_USDC_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("ENUsEs9bV6SuSd1iYu8HaxdnoXSv1ZBU3zc5os3sKqSi");

/// vault address account for the ALEPH_USDC farm
pub const ALEPH_USDC: Pubkey = static_pubkey!("J8tCBKFagSAg1FG425pzLmyyP7xgoRN5bL1W1ztGLHir");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const ALEPH_USDC_INFO_ACCOUNT: Pubkey =
    static_pubkey!("3DSDfQQJKfyq3zyod6eApZRUyEwXeUnCtbdamGNAxe9k");
/// vault lp token account for the ALEPH_USDC farm
pub const ALEPH_USDC_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("5s2dEjXkwHETBNr6ybPURmDF6MdejVgTvMFRXG5aiJFN");

/// vault address account for the TULIP_USDC farm
pub const TULIP_USDC: Pubkey = static_pubkey!("BX7otwshaKGXm3BKzyvy3dPz9wmqAK4Jn3t4kGBJEfKy");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const TULIP_USDC_INFO_ACCOUNT: Pubkey =
    static_pubkey!("9A2hWEUYb2bihcjioGxJohjy7ZFEBDcUbxkhKb8WzuNd");
/// vault lp token account for the TULIP_USDC farm
pub const TULIP_USDC_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("8CHUanu6wrgyW6VSqZcHvSpaipg4ZrPEcvbt7xmD99P1");

/// vault address account for the SNY_USDC farm
pub const SNY_USDC: Pubkey = static_pubkey!("7uy6iA1i7JRUm4gZCd6njZddDPbudPqhvw5vrZ8FNiWM");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const SNY_USDC_INFO_ACCOUNT: Pubkey =
    static_pubkey!("9F84NCL736ig9Bke8Fdc6pqVc5zbuJzN7q9BGVPzYrVt");
/// vault lp token account for the SNY_USDC farm
pub const SNY_USDC_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("AhGbpRkyMrUJEMBupwnGd6mxogyzNZSQB8nP7RqDSsYh");

/// vault address account for the BOP_RAY farm
pub const BOP_RAY: Pubkey = static_pubkey!("7TRJe35BgP7LHtzFDFAd4ZpUijcQ6oSXofrx4ikc3s6b");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const BOP_RAY_INFO_ACCOUNT: Pubkey =
    static_pubkey!("8hUsgFBpGqkeqE7b6poovTP7JaudYbwbyAujjfECDLrq");
/// vault lp token account for the BOP_RAY farm
pub const BOP_RAY_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("5HfZtRdhE4M5UyUAT8yW4qsMEd6jAVuVEvPy6K5rJZgJ");

/// vault address account for the SLRS_USDC farm
pub const SLRS_USDC: Pubkey = static_pubkey!("26Tp4c1V1pAJzYyTDoZhSHwDSQ8gcZvuaVeztMFUSJgp");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const SLRS_USDC_INFO_ACCOUNT: Pubkey =
    static_pubkey!("BEBADzLL3Nzg2yvptygNAkqQnWjK7Jyg1GZskcw42n67");
/// vault lp token account for the SLRS_USDC farm
pub const SLRS_USDC_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("C5oDu4aHj6zLYa2Jtv8AZwVGivBvRzMDARpAkey1nTfV");

/// vault address account for the SAMO_RAY farm
pub const SAMO_RAY: Pubkey = static_pubkey!("AgtFUci2zTkG2pf36Do6kiq6xfyn7jDy7GYhqg2GrVw7");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const SAMO_RAY_INFO_ACCOUNT: Pubkey =
    static_pubkey!("58VWSh3Hh8F6wqSDW7SzXqCFWXaib1NtKR6DXdGE8Qbb");
/// vault lp token account for the SAMO_RAY farm
pub const SAMO_RAY_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("5t7T23bUUGBmX9KLv1PfBMTY9UfNhofmZuVELPcXf1HC");

/// vault address account for the LIKE_USDC farm
pub const LIKE_USDC: Pubkey = static_pubkey!("C3grzmHJTK5spXnuYYZvpQzz2pC9ZAWBj7nciy3aegWW");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const LIKE_USDC_INFO_ACCOUNT: Pubkey =
    static_pubkey!("48mvZpVhJwpHLePFB2Aa2CMqBYgzwUAmB59hCSUDVfev");
/// vault lp token account for the LIKE_USDC farm
pub const LIKE_USDC_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("JE6vMmwb61ZZ9Pae6yPXoyCxYKCbAu2Yt87XdJs1GTNc");

/// vault address account for the MNGO_USDC farm
pub const MNGO_USDC: Pubkey = static_pubkey!("B57LVNfNHqFDhaC2ayLGNfGN1QdMXgJzdkESApGLfRkH");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const MNGO_USDC_INFO_ACCOUNT: Pubkey =
    static_pubkey!("HkEk1S6oEY2Mki7xUtwnHVrQzD4jycwahmtWYbjuPD86");
/// vault lp token account for the MNGO_USDC farm
pub const MNGO_USDC_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("HKz1xwgW6sTCVy56H2HiuUtqECxDBYTinjaZKERAucUe");

/// vault address account for the COPE_RAY farm
pub const COPE_RAY: Pubkey = static_pubkey!("F1uJw28VmB5HHnMWW6Bgz6LVsd4ApjK462FANHtGQGJC");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const COPE_RAY_INFO_ACCOUNT: Pubkey =
    static_pubkey!("9EGieJ2duSG73EcHATJ6ZVLeXEdt4H9BPrZozt4g7GLp");
/// vault lp token account for the COPE_RAY farm
pub const COPE_RAY_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("4CwpPi2Ng61LWMiXJuzP497R4u3EZTQFQ8cMQFdtaj72");

/// vault address account for the LIKE_RAY farm
pub const LIKE_RAY: Pubkey = static_pubkey!("3cMzVu55tLrDh3eYxtVYqwLS13CDv1gjZ6h7wUX1YfTW");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const LIKE_RAY_INFO_ACCOUNT: Pubkey =
    static_pubkey!("4GMd7JqH3p1aK1yhim7AirnScJN48xpCbpatYmnRHK67");
/// vault lp token account for the LIKE_RAY farm
pub const LIKE_RAY_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("FwqM4ZXxwzApWML9vyJoJigrwQxNJ9HzH6kPfdAXFzzW");

/// vault address account for the SNY_RAY farm
pub const SNY_RAY: Pubkey = static_pubkey!("C9YtaCoaTtNzygQrtqS2iZ38HaUzGv6qDzToCA5Z3aae");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const SNY_RAY_INFO_ACCOUNT: Pubkey =
    static_pubkey!("44V1uhMMgkX1F2AZb8BJxVYasWgAKxc7JFA7NqXkqHar");
/// vault lp token account for the SNY_RAY farm
pub const SNY_RAY_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("BP7d3jEYS8BhnGmUEma1PBDpuP745KTbCK9mQPd68ocd");

/// vault address account for the MEDIA_RAY farm
pub const MEDIA_RAY: Pubkey = static_pubkey!("3PoBB4pvYUAHeKqd9CUpecBJ3NQS2g5JCVnBEtUdhFGC");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const MEDIA_RAY_INFO_ACCOUNT: Pubkey =
    static_pubkey!("CtxtfmmGsdCUEme6tcCcUfQ57zuvZ7kGuxggRCwojXw7");
/// vault lp token account for the MEDIA_RAY farm
pub const MEDIA_RAY_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("8tvcyNjp8SWGeHaon6VngT7tqmM8hLs2YvsYigRRiX26");

/// vault address account for the MER_RAY farm
pub const MER_RAY: Pubkey = static_pubkey!("Eiu8GgGm9dKkzVYdgQcwTm6rjY1fTYcBDX5SwKbuUBD");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const MER_RAY_INFO_ACCOUNT: Pubkey =
    static_pubkey!("Dmh6udBebNAXd5P5Guqj11ogB1Z3uEJCb1wRTHvKZ3CL");
/// vault lp token account for the MER_RAY farm
pub const MER_RAY_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("ES8SJ9FyCz4b2apZNSKwD5tPzcoBoYEnBvhNDhz4ifru");

/// vault address account for the SLRS_RAY farm
pub const SLRS_RAY: Pubkey = static_pubkey!("Bgf9StDoKx6eF2sWxSHLZnBKQ5woCRoxbhcoB1gu2A33");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const SLRS_RAY_INFO_ACCOUNT: Pubkey =
    static_pubkey!("9SYLgCZdUGLNe5iTxsJ4USUWhCd1RwVuNt1jbZKLaMM3");
/// vault lp token account for the SLRS_RAY farm
pub const SLRS_RAY_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("4mtxt4BVTrSwj4Q3ZvMuM1MFk8cEbi6T7RkGCwPuiLqA");

/// vault address account for the ATLAS_USDC farm
pub const ATLAS_USDC: Pubkey = static_pubkey!("CyLryeeJRgqDzAbKS66kEg9tXD9JQAo4Airp24KTQiSH");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const ATLAS_USDC_INFO_ACCOUNT: Pubkey =
    static_pubkey!("A9LZbz7DhhYRbxzuLw9CCa5B5d63e7SVGiSCGVwnAjkx");
/// vault lp token account for the ATLAS_USDC farm
pub const ATLAS_USDC_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("9PWMzgd6k4QMk35P8BwmFyTmiS1MB7hSCvkNaMu1pLod");

/// vault address account for the POLIS_USDC farm
pub const POLIS_USDC: Pubkey = static_pubkey!("99YLQuPAUwrja8Datay1TxHsuFCJ6YwMAo4m7sB4LrnM");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const POLIS_USDC_INFO_ACCOUNT: Pubkey =
    static_pubkey!("31G5RkPXqFY1J1PF7geH1sT7fwi618ndTS7oie4USd31");
/// vault lp token account for the POLIS_USDC farm
pub const POLIS_USDC_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("7JWqn3ZVFQ4nMbS3Lqqoijz1SF9CM6fL1RUmj69QEGmx");

/// vault address account for the ATLAS_RAY farm
pub const ATLAS_RAY: Pubkey = static_pubkey!("62Y31eYBcxinJRaMqDd7mMmrtC8mxNbVnLfqQSmE8KRd");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const ATLAS_RAY_INFO_ACCOUNT: Pubkey =
    static_pubkey!("6KVDKLj38SyFL4QwxpiUptqZ13ZcEkFgKU4ydfn79XGy");
/// vault lp token account for the ATLAS_RAY farm
pub const ATLAS_RAY_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("5ceU7V4HDUu2kcq4eXyA6zExqgMFH4fd8SLuGQe4qiBC");

/// vault address account for the POLIS_RAY farm
pub const POLIS_RAY: Pubkey = static_pubkey!("BXhcM86zbyrnbG5sdaQTwuueCUHwFVFC61TXwKhSqCvc");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const POLIS_RAY_INFO_ACCOUNT: Pubkey =
    static_pubkey!("zKoZafMwd1WFNKZYGKCF8pK95V9MRWfxLYf1CKxoAKz");
/// vault lp token account for the POLIS_RAY farm
pub const POLIS_RAY_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("2nSAD2v2JZMY8HuZfL7UgzAJEh54WeRPJYm5QNXd6UJm");

/// vault address account for the GRAPE_USDC farm
pub const GRAPE_USDC: Pubkey = static_pubkey!("GDy1S6a7VYgjuECSj2hJsoZ7BexSn23jHFjk5fYBpgi5");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const GRAPE_USDC_INFO_ACCOUNT: Pubkey =
    static_pubkey!("C4bQFQ88k68CeT2eZVMBTRT1k8gdgXoDEAPkr4VKtgKM");
/// vault lp token account for the GRAPE_USDC farm
pub const GRAPE_USDC_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("9VXQ6WgCbPAAbzTmKZRF5VTXzs8zUEoYwDuo9FVM2Yd");

/// vault address account for the LARIX_USDC farm
pub const LARIX_USDC: Pubkey = static_pubkey!("9ZgrNHuRm5NPQDKJYipxcR6WXqFmUPLDuUSKEZZ8a36G");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const LARIX_USDC_INFO_ACCOUNT: Pubkey =
    static_pubkey!("5mLeQo6wpkMRHizB1NcMzF3KKWVfE6sRsHnKGZZUZ1hq");
/// vault lp token account for the LARIX_USDC farm
pub const LARIX_USDC_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("B19zSrVWudQw6ULU2sDJ3yYRThwA7bzKTpPPR7BNDgdN");

/// vault address account for the mSOL_USDC farm
pub const mSOL_USDC: Pubkey = static_pubkey!("GfM1JxpucDcgoWgSbU5usgAAuQF8wddkpZZNoF4mjXpk");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const mSOL_USDC_INFO_ACCOUNT: Pubkey =
    static_pubkey!("BZR7uNzz3LuLv7eSjaWJrN4ZYZapTV8eYyCqfEZScmTE");
/// vault lp token account for the mSOL_USDC farm
pub const mSOL_USDC_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("Do5CDaoHforNxUrpuVpTyNpVWogreF5Gt5HpNYXo2jLw");

/// vault address account for the mSOL_RAY farm
pub const mSOL_RAY: Pubkey = static_pubkey!("HubM6Fa48GAy1K1jdW1iRribDGLHzLwNrNUVKWjdP5FU");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const mSOL_RAY_INFO_ACCOUNT: Pubkey =
    static_pubkey!("8sBb3tdVd3X3THPU4mKjkxFoUSAhCV6oyKio6DrqBjQ6");
/// vault lp token account for the mSOL_RAY farm
pub const mSOL_RAY_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("FicV1u17BNTL2JToj51Eu5egeLmkUf2NJ9usuYvs5qxK");

/// vault address account for the LARIX_RAY farm
pub const LARIX_RAY: Pubkey = static_pubkey!("7t8j9KiPNNWVLWVDsNNouK2Nk97KdQuB45y8tT3ag3TQ");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const LARIX_RAY_INFO_ACCOUNT: Pubkey =
    static_pubkey!("GpEjNW6Hxc6KRQGijaE8toFoxgYo5K4aRBbN7w99xK3k");
/// vault lp token account for the LARIX_RAY farm
pub const LARIX_RAY_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("5M14vctrYezr5d5jEYbaNPiXEu2128Yro56EYuLs6U1a");

/// vault address account for the MNDE_mSOL farm
pub const MNDE_mSOL: Pubkey = static_pubkey!("72PfMr7V2bBtoBTA6rhtnrKikpduHjcNR4AJiiHb5LDD");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const MNDE_mSOL_INFO_ACCOUNT: Pubkey =
    static_pubkey!("FDM8k8m89A2SvB8aWEzSe68RmS8eFrL6mdQxcq9WWu5K");
/// vault lp token account for the MNDE_mSOL farm
pub const MNDE_mSOL_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("CNDzf2Ksst2VjcwLz7TwSsNtdhq9qLw67Cr4tv4REZLj");

/// vault address account for the mSOL_USDT farm
pub const mSOL_USDT: Pubkey = static_pubkey!("BKo8Pj1hWa264e49ji18mPrxPLgDgZGU4bsUdTEwC1TA");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const mSOL_USDT_INFO_ACCOUNT: Pubkey =
    static_pubkey!("2572yGjYEyxyWLGFZ1G7Cq26uQVnzN8ufa83QkwjifxM");
/// vault lp token account for the mSOL_USDT farm
pub const mSOL_USDT_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("GVTsRmMpFg4rkHUYsEfeTNTxHg6tRcmTBcAJHLYp9eT5");

/// vault address account for the ETH_mSOL farm
pub const ETH_mSOL: Pubkey = static_pubkey!("9igj1xB4qW18WPQpNY7AGwcJQniT4XxKy4hptUkBJ52T");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const ETH_mSOL_INFO_ACCOUNT: Pubkey =
    static_pubkey!("3qQA2g9d26TuEnH5fAWzgY92PA9jM77DcSJpXE5GoiYM");
/// vault lp token account for the ETH_mSOL farm
pub const ETH_mSOL_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("9DoGS93s964Md73eBSS8iP4ecN4eiKhby6rN1Z8RpXfv");

/// vault address account for the BTC_mSOL farm
pub const BTC_mSOL: Pubkey = static_pubkey!("2FdRTD49aPBtvVERvV4VUxD6At7jRNvSfezT3A8tPy2M");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const BTC_mSOL_INFO_ACCOUNT: Pubkey =
    static_pubkey!("2xvpDszS5AFLYU5kC4v5VUGxddwHH4tgXizqrMd8MU7q");
/// vault lp token account for the BTC_mSOL farm
pub const BTC_mSOL_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("4wsNkhy8sTYyWUSKsgadae26f4q4cgBvoGTN3GAn6xSB");

/// vault address account for the LIQ_USDC farm
pub const LIQ_USDC: Pubkey = static_pubkey!("zKWDi7pZYEkwjDnDryeyaTEgXNkCPQGV9BMT6g1qhSA");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const LIQ_USDC_INFO_ACCOUNT: Pubkey =
    static_pubkey!("4TWdNyCCtsRMrjXac8J5Xk6HsKkSngCpstuPyD4shFKc");
/// vault lp token account for the LIQ_USDC farm
pub const LIQ_USDC_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("DgTuBZG5iP1ZkhgRA7MU8F2cMVVyiF5uLo5hMWYMgHU6");

/// vault address account for the LIQ_RAY farm
pub const LIQ_RAY: Pubkey = static_pubkey!("F9ZGSmYxLbx4DR5J85DrqQrdLmZSGS9s5EivVz1EEkrS");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const LIQ_RAY_INFO_ACCOUNT: Pubkey =
    static_pubkey!("4gT3emw4cezC3FDbespznqCtSSvQRbuubEtsrPzoVyEH");
/// vault lp token account for the LIQ_RAY farm
pub const LIQ_RAY_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("BaXqWqscuvKHvLc7vVwMwk3auf2WM6Zy41aawgqUZij3");

/// vault address account for the SYP_RAY farm
pub const SYP_RAY: Pubkey = static_pubkey!("HdfwFZ6MhEXEbXboEmpF85H31DPGBBqGdSh4Xw5xHB2J");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const SYP_RAY_INFO_ACCOUNT: Pubkey =
    static_pubkey!("6eCQcxQFPd7kKE6MXcv5gXg4Uw2gcoKtoSwTUAtFmBG3");
/// vault lp token account for the SYP_RAY farm
pub const SYP_RAY_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("Bn9HxN8Nj5sKXYS2cTzkZsq4zqAHqM1jv3iVp7wfNZF7");

/// vault address account for the SYP_SOL farm
pub const SYP_SOL: Pubkey = static_pubkey!("BLmEV9zg5w1rcaM96vCG2QgHpeBtZyRsvL8B2aUdD77E");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const SYP_SOL_INFO_ACCOUNT: Pubkey =
    static_pubkey!("DJxL7RRp2JbNbvwgjuf1kJMbHg5Gf4FnrZagty6PJBoZ");
/// vault lp token account for the SYP_SOL farm
pub const SYP_SOL_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("C2FNsb9UEouviMdTNaK4SX1DV2YssQpHBGCkCx7xjtcs");

/// vault address account for the SYP_USDC farm
pub const SYP_USDC: Pubkey = static_pubkey!("5XHRZdnDnq5BKWL96WDS7uyi5yaaSCz3RTEUzWKsoBVt");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const SYP_USDC_INFO_ACCOUNT: Pubkey =
    static_pubkey!("DGhqjUSJ1FZuGRBfupa78LdoMhnE9TM5K5KntMdno3Gh");
/// vault lp token account for the SYP_USDC farm
pub const SYP_USDC_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("EsaWX4QEL94GRFqtuAQBVb2UERyDaHR74LFCEiSgJJyL");

/// vault address account for the WOOF_RAY farm
pub const WOOF_RAY: Pubkey = static_pubkey!("D4uJ4Gri4Aq9P5s7YYLjzjGdHpQVC2fnAm6JbYR5YS9T");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const WOOF_RAY_INFO_ACCOUNT: Pubkey =
    static_pubkey!("CVw3PF6r4xAJJTxWxdTvEMxuBM17iYzs9m7tg4HfWZWw");
/// vault lp token account for the WOOF_RAY farm
pub const WOOF_RAY_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("7Ctzyqt1QPWsuPYPWFCAqBkqtbWfJkjGYARU4rb3m4yH");

/// vault address account for the FRKT_SOL farm
pub const FRKT_SOL: Pubkey = static_pubkey!("3UCHNmLSAPzv8n22CbcCGcAzKZF2D7uw64VCyeA7eCHX");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const FRKT_SOL_INFO_ACCOUNT: Pubkey =
    static_pubkey!("5TfUqeDWKiqXKgMXr6CDTSwS5bUr1tir4nfwuifWCAdu");
/// vault lp token account for the FRKT_SOL farm
pub const FRKT_SOL_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("8pfYJRMxt1eBXXQDgJhf7GYeNjjCxRjhWnPqQocoHAvC");

/// vault address account for the whETH_SOL farm
pub const whETH_SOL: Pubkey = static_pubkey!("Cut9wFgJTADDuNdyqi6Nbahtjvg1Yh8T1asEYvTQgTtu");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const whETH_SOL_INFO_ACCOUNT: Pubkey =
    static_pubkey!("4jCYTLg2nxQB979MF1ByEfEqnPivmN8kYTmANm6zuEU5");
/// vault lp token account for the whETH_SOL farm
pub const whETH_SOL_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("HrP6bzGnkkpeSdMytqM6NuE1U8oEhH4dCLw9CMwuAG5T");

/// vault address account for the whETH_USDC farm
pub const whETH_USDC: Pubkey = static_pubkey!("7Yh9GX2bXjxFjJrGvPCh1YbNuLQXoSDPHLtVBCy3o9ka");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const whETH_USDC_INFO_ACCOUNT: Pubkey =
    static_pubkey!("Eg57xeD9Fgy7npthsnAx4dtJALzvH7AkK5C88eBhE1ah");
/// vault lp token account for the whETH_USDC farm
pub const whETH_USDC_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("7CJdQp5WfzrviKTuUPUwU1PK6E5KRu5S8ewtKoDiPciC");

/// vault address account for the weUNI_USDC farm
pub const weUNI_USDC: Pubkey = static_pubkey!("NDwLeteyAbtijvc8eRNijpHuvyzTR4Dg167Y2bScyp4");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const weUNI_USDC_INFO_ACCOUNT: Pubkey =
    static_pubkey!("DPud8kUTXqNSehqc3UQVWpNWYs6gDABCrQDm5b7jZhrR");
/// vault lp token account for the weUNI_USDC farm
pub const weUNI_USDC_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("65tDxBmj4mBitPpK72tonuogbDsn4tkEDDUUBgMHQ2or");

/// vault address account for the weSUSHI_USDC farm
pub const weSUSHI_USDC: Pubkey = static_pubkey!("D1AHVWKFRZBEt7jWjx13xAgTA2wxQ7CXieUVjAE6f2ML");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const weSUSHI_USDC_INFO_ACCOUNT: Pubkey =
    static_pubkey!("2EeboYUWzhbpxKAo2ym7UhfVL5BX9hAYTwFMhraSkfUd");
/// vault lp token account for the weSUSHI_USDC farm
pub const weSUSHI_USDC_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("FwPYoMoLW6ZisJuNMiQWCZGUjsRK36DXzUT5PYA8hppZ");

/// vault address account for the SRM_USDC farm
pub const SRM_USDC: Pubkey = static_pubkey!("sEboBJkciTicDmqsGzdhyd7WpMZzXzk7wNnkLMTcVGc");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const SRM_USDC_INFO_ACCOUNT: Pubkey =
    static_pubkey!("BMKgW73bCdrJ8MiyMBqbvUDipZeouiQLkfCcqu3HVDqu");
/// vault lp token account for the SRM_USDC farm
pub const SRM_USDC_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("14b19WHsLzUFE2Xtjtv6C77m9FrMMFzVecqgzwpqC4p8");

/// vault address account for the STARS_USDC farm
pub const STARS_USDC: Pubkey = static_pubkey!("54mVgifTiS2yfRSn4LQGHMcqXrHYx6SXPsCcHb2M4txo");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const STARS_USDC_INFO_ACCOUNT: Pubkey =
    static_pubkey!("7i2dVZAEQ5ZgcZ3uyoFZqFac7XGuNAmB5bZV8RzH45Qt");
/// vault lp token account for the STARS_USDC farm
pub const STARS_USDC_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("8WB1DnebeSoEy6jcS67RCzYQ1M5dfjmSPHv3mU3cR4P5");

/// vault address account for the weAXS_USDC farm
pub const weAXS_USDC: Pubkey = static_pubkey!("ZpmbhMkqNdqAZxsjPWsWeKMvkXSk7T5QRMAiAiZLKee");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const weAXS_USDC_INFO_ACCOUNT: Pubkey =
    static_pubkey!("FGrkZWpr7CAzX4oE1nTkBwk3soa7jrfDqT3SnGTVXtTD");
/// vault lp token account for the weAXS_USDC farm
pub const weAXS_USDC_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("6hq93WvUBM4fvqXe8odAyD9ouw6DJU7WkJYii5EaUoA4");

/// vault address account for the weDYDX_USDC farm
pub const weDYDX_USDC: Pubkey = static_pubkey!("HoVq5LcHv3TMVCEVJQrobgAQSWwyH5btZAJsR8fU1rCH");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const weDYDX_USDC_INFO_ACCOUNT: Pubkey =
    static_pubkey!("H7S55FY5QWEy68wpuRPjwK1L7dB65mGwYhKz4rP9EmWu");
/// vault lp token account for the weDYDX_USDC farm
pub const weDYDX_USDC_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("6TNQ9w4zJWNbvyPuXnKEU2jtvbwJf7vNSW65o84kkte9");

/// vault address account for the weSHIB_USDC farm
pub const weSHIB_USDC: Pubkey = static_pubkey!("4PMRucjmVNMTaxW14Br9RaiwqbVZAo9Bnz3QhEHiSx2T");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const weSHIB_USDC_INFO_ACCOUNT: Pubkey =
    static_pubkey!("GEcTJKA5apCNACXmiXNDKegpmtBLko2oXeY8M1sF7qXh");
/// vault lp token account for the weSHIB_USDC farm
pub const weSHIB_USDC_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("Ht7s4ovPWB2g434de6WrQqUxqTUTwzKXcmd5n86xrHkQ");

/// vault address account for the weSAND_USDC farm
pub const weSAND_USDC: Pubkey = static_pubkey!("ByMs8ia6XL5zRskMWXBb8BTLcgxnLjqhg5FxVy93wBPV");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const weSAND_USDC_INFO_ACCOUNT: Pubkey =
    static_pubkey!("BECvHu35gpnkvux6PMFPC2UMsYePuXcuHs8e1Dx9wmQ8");
/// vault lp token account for the weSAND_USDC farm
pub const weSAND_USDC_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("2KX7x5Dh7VD9F1iQZoQ1TfncmUccjuK7KRJCUxj41MNx");

/// vault address account for the weMANA_USDC farm
pub const weMANA_USDC: Pubkey = static_pubkey!("EkTorhDMRv8ueEUyLabwG3vvFJPzjNTen9kQ6aMdBWjs");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const weMANA_USDC_INFO_ACCOUNT: Pubkey =
    static_pubkey!("8pRqP95ARRVcCrXRfdnapudZBJJ79s1gN8VzQhsyxaDE");
/// vault lp token account for the weMANA_USDC farm
pub const weMANA_USDC_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("E4UU752jEDE6iHM7NnndR7m1qFUMqmzgvTyEpCLEV9po");

/// vault address account for the CAVE_USDC farm
pub const CAVE_USDC: Pubkey = static_pubkey!("ApzURY9RNAanfBszeGH3PEGW2Dg3WRfysKEt7xVe2Xx");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const CAVE_USDC_INFO_ACCOUNT: Pubkey =
    static_pubkey!("3JMUsWwauTP5obAiUy36e9u1Q4YmfJcWpP8SXuay6AVe");
/// vault lp token account for the CAVE_USDC farm
pub const CAVE_USDC_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("HsBbACpj7nD39GLKSxWXmgavS1H5c1jxCSF4czzgDg1Y");

/// vault address account for the GENE_USDC farm
pub const GENE_USDC: Pubkey = static_pubkey!("Gky9Y8F5rgF4pcBypwX6Aoe8Zrw9hiRWGDrDhg2zaUu2");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const GENE_USDC_INFO_ACCOUNT: Pubkey =
    static_pubkey!("BowkGmh5oZcNCBVeYbnKxwfamBBd7iHMUdPWYaPNWke");
/// vault lp token account for the GENE_USDC farm
pub const GENE_USDC_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("DWHgcn1bf5NWPTKPKaSdfHCGBdmVamyvkw4ueMvwZ2G3");

/// vault address account for the GENE_RAY farm
pub const GENE_RAY: Pubkey = static_pubkey!("6yZxgmyVVxshjGzPsLScnAdEL6P6j4NG6qzPp4jqRofy");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const GENE_RAY_INFO_ACCOUNT: Pubkey =
    static_pubkey!("37ddXWj99mYCtrMNmH1tZjuw4RwpYahjCDPYgqFsAYuU");
/// vault lp token account for the GENE_RAY farm
pub const GENE_RAY_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("6FUYcKxhADJDD2dSZoUbViwmECY1j4EP43i9sQXoUXcQ");

/// vault address account for the CWAR_USDC farm
pub const CWAR_USDC: Pubkey = static_pubkey!("52vjXtCTSfhKhgXYw9kue2reeyyWaDLQ5gTktyVZNNZQ");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const CWAR_USDC_INFO_ACCOUNT: Pubkey =
    static_pubkey!("GZbdxdYt2k8XDoTTSZ8bXSvGDxTJ67egyWGixMQQ3Rr1");
/// vault lp token account for the CWAR_USDC farm
pub const CWAR_USDC_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("DUTviYhfMEhCd2QBL7TQFM3QrkJX8X1YSFdGBPka6XRA");

/// vault address account for the SONAR_USDC farm
pub const SONAR_USDC: Pubkey = static_pubkey!("Fgjojzw7w86Avap8jpi696bW7awgSFo2cZMAdjJ5bPcD");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const SONAR_USDC_INFO_ACCOUNT: Pubkey =
    static_pubkey!("6WRn5JTHnWfJiRvYCHABuDBptXNkXMHcxbuQye8oUvWs");
/// vault lp token account for the SONAR_USDC farm
pub const SONAR_USDC_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("7AR1oC4MpMJ5g1HLPGfUAkWaT93nnb2Bfie1ySKjpNxc");

/// vault address account for the SHILL_USDC farm
pub const SHILL_USDC: Pubkey = static_pubkey!("AHExXA9FJu913sBbD31TjGtKkkJvDNwvyVafaTgVnWaa");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const SHILL_USDC_INFO_ACCOUNT: Pubkey =
    static_pubkey!("HWcF5Ppxwv9ZPWsANRswYGcvtoKji7pyD3PSLo425fLR");
/// vault lp token account for the SHILL_USDC farm
pub const SHILL_USDC_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("8LU6VXUcd44dDCK5XPRzSRq7TZoeHBX6P4E6AXaAwcNF");

/// vault address account for the DFL_USDC farm
pub const DFL_USDC: Pubkey = static_pubkey!("5TvsqP5DQEJPqP3atsw337uPDcrGJnTLC3vsxTADQGgi");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const DFL_USDC_INFO_ACCOUNT: Pubkey =
    static_pubkey!("9gGRzRg9PGr5VhGuphufjM5mvkpT8NnUiaZy9bDVk67G");
/// vault lp token account for the DFL_USDC farm
pub const DFL_USDC_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("2uyfY2GuGtei2yr11HdUmqX3XCuVYnh79SyCd4wUn8Dz");

/// vault address account for the APT_USDC farm
pub const APT_USDC: Pubkey = static_pubkey!("3BXRN4ENbvEBqFBDmyc16cxfiyh8SD1evbhCMFCN6XED");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const APT_USDC_INFO_ACCOUNT: Pubkey =
    static_pubkey!("5c3EXxYMLztc7gPGzCnqxZXJBQ1ySUZpnxUP17qmUzzm");
/// vault lp token account for the APT_USDC farm
pub const APT_USDC_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("BtAU9TKKCUSQ39vCS1pwzyThnYuSxGomsntSAQRnak9o");

/// vault address account for the MIMO_SOL farm
pub const MIMO_SOL: Pubkey = static_pubkey!("BZHV67CnPJmuEyMAzjfy7b1fNYds5yiGCSV5Gp4LSS58");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const MIMO_SOL_INFO_ACCOUNT: Pubkey =
    static_pubkey!("E8SRCYkWPzhJiwFKtMd9yD49turFwf3FUwpSzQZRkh9D");
/// vault lp token account for the MIMO_SOL farm
pub const MIMO_SOL_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("4wTiTwHEzhbsBdBJ1weD8jCTCvXKCVvMESQuH6KjAMCe");

/// vault address account for the wbWBNB_USDC farm
pub const wbWBNB_USDC: Pubkey = static_pubkey!("9iYjLDq45RgbxCkiGtHJvmvQtjdsibotukT3wcB1eqm7");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const wbWBNB_USDC_INFO_ACCOUNT: Pubkey =
    static_pubkey!("4Tdvsx3UK1YRLK5gcZSW7oy68Cj6HWggA9y5PXgdUYW7");
/// vault lp token account for the wbWBNB_USDC farm
pub const wbWBNB_USDC_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("4dXH7PTJiHjwEenuP3YKnR4nA2JZYvVxET7FNfQnw2N9");

/// vault address account for the wePEOPLE_USDC farm
pub const wePEOPLE_USDC: Pubkey = static_pubkey!("4YmtJW213waqMgocGvKbuW1ubD7Kx7wWFhfXxA5fRGNk");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const wePEOPLE_USDC_INFO_ACCOUNT: Pubkey =
    static_pubkey!("8S6hbTtK8CfMeLYVBZ6BPcYyx3ArGG4FPMG43mo94Btx");
/// vault lp token account for the wePEOPLE_USDC farm
pub const wePEOPLE_USDC_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("DWVpMUxBFHiPpbudRwEV7cvFWkDrdWHD7Uznz9CC4zDT");

/// vault address account for the BOKU_USDC farm
pub const BOKU_USDC: Pubkey = static_pubkey!("GS4RuMAFErqaSoBtxdpR7u2LNeiFinZ3Tmnx8kNRknQ7");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const BOKU_USDC_INFO_ACCOUNT: Pubkey =
    static_pubkey!("8kcDtaBoAkhLaM7afLQETYtoRdVid32YPV7zcCVjUjSw");
/// vault lp token account for the BOKU_USDC farm
pub const BOKU_USDC_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("6eQF86opZ8tjRg282NRWTZGvV3D81p9yCykoexGT8vcG");

/// vault address account for the XTAG_USDC farm
pub const XTAG_USDC: Pubkey = static_pubkey!("7AzfiHriEBronjgQStJx2aAeiCg8HLs2xbKwB9tmtUjH");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const XTAG_USDC_INFO_ACCOUNT: Pubkey =
    static_pubkey!("BZKjqYaxegAPtRitkJ4JDA7MzhPwqGHnod5AYWWtcHPC");
/// vault lp token account for the XTAG_USDC farm
pub const XTAG_USDC_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("AMq6vSFeWjpUsFNgAiTHhHfLU2MJPW2GqtkqrRZxR6Am");

/// vault address account for the TTT_USDC farm
pub const TTT_USDC: Pubkey = static_pubkey!("FPZ3njeioHU28aHd4hkXGeL3pSXxvcvcGnveYmjd17nZ");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const TTT_USDC_INFO_ACCOUNT: Pubkey =
    static_pubkey!("F5WSYeACaini242BYB2kXTEWDWkQwQcfyuWYYs6q77M3");
/// vault lp token account for the TTT_USDC farm
pub const TTT_USDC_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("F6pU8q2V8aFBmZDb286LnHxEko9zVutfYtfYUVCFrhif");

/// vault address account for the SOL_USDC farm
pub const SOL_USDC: Pubkey = static_pubkey!("91M42pKURwf4VQHACzx1VFZ8PGZgW2RDwPkwbBk8peGU");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const SOL_USDC_INFO_ACCOUNT: Pubkey =
    static_pubkey!("HB1FUY2CkPC9W4xrc8FLEbzTwqo23ErBRGuLwoaoAdYW");
/// vault lp token account for the SOL_USDC farm
pub const SOL_USDC_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("DS2FURhtbN4fvUfKmq32Eq3uxywv5JSuYXfd62sncZ3L");

/// vault address account for the SOL_USDT farm
pub const SOL_USDT: Pubkey = static_pubkey!("CuGdRkbBbHrHEMt7b8RSqTyuW8wkXqMtLGdmopdmWENm");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const SOL_USDT_INFO_ACCOUNT: Pubkey =
    static_pubkey!("GUkBo8tR5CZL64jasRo4PzugvQLRdPLvsjrV8peqPLZR");
/// vault lp token account for the SOL_USDT farm
pub const SOL_USDT_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("BfWFjyfTWMyeS56zJZEwhwr2cVgTzXvVuC6sxKSAPrAM");

/// vault address account for the RUN_USDC farm
pub const RUN_USDC: Pubkey = static_pubkey!("42G6xTpSG5BtdMbDNxL8gNouktE6Azz2xV1U6FiUXYh5");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const RUN_USDC_INFO_ACCOUNT: Pubkey =
    static_pubkey!("DgjP64FENRZFyiGWjbTpqNJECwSGdmjAAqZxsbub6iMu");
/// vault lp token account for the RUN_USDC farm
pub const RUN_USDC_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("FWKFSSPAwKZyiYoWKhuJZhiFaLFANDiNsr2J84ENcBgU");

/// vault address account for the CRWNY_USDC farm
pub const CRWNY_USDC: Pubkey = static_pubkey!("6aLTuS5naAjKo5vbB4FQS8SzVKwLqNZwJDmt3MtQX8NW");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const CRWNY_USDC_INFO_ACCOUNT: Pubkey =
    static_pubkey!("2uNAFiRwZo7gFQMTHg25gccwfu3UNppZAvi2o65mRpRV");
/// vault lp token account for the CRWNY_USDC farm
pub const CRWNY_USDC_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("9ogHjigzHe1VvwN2XFiH9iBdUJZVi91TVCK2oHetzBMk");

/// vault address account for the CRWNY_RAY farm
pub const CRWNY_RAY: Pubkey = static_pubkey!("Ak8qEjMozRJXjfSDopdEqwZxtEGjdwTFRMHgxcAYDnJm");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const CRWNY_RAY_INFO_ACCOUNT: Pubkey =
    static_pubkey!("5yfzaTKscXj5DCkfeGu1aB4MN8xNqkvCz6FAeNpXxAME");
/// vault lp token account for the CRWNY_RAY farm
pub const CRWNY_RAY_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("DoXXtqnemAHHgmNKurirZqT3hwyFEiZXtE5LzqmDccmn");

/// vault address account for the MBS_USDC farm
pub const MBS_USDC: Pubkey = static_pubkey!("DpvWmk7t5FzQVGSt3QbjzYA4mSXvHXB1Nt9dikBEC4JE");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const MBS_USDC_INFO_ACCOUNT: Pubkey =
    static_pubkey!("d4PY6JcwDAACAg3SniCfYNF44zD14LqYMwwKZmSCcTi");
/// vault lp token account for the MBS_USDC farm
pub const MBS_USDC_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("GgibjTCAJPoATngBbyLbGtZVfCn89nuGhcaED7V512eF");

/// vault address account for the PRSM_USDC farm
pub const PRSM_USDC: Pubkey = static_pubkey!("HGCrMw4zZX428EAzcS9TfFpMhyvhQo4FTTwp9yn7gngC");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const PRSM_USDC_INFO_ACCOUNT: Pubkey =
    static_pubkey!("7CukktEpSwQFbEumyPqFmhMmk77Fg3CMHef7UTrS8XeA");
/// vault lp token account for the PRSM_USDC farm
pub const PRSM_USDC_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("7kVjpkynCgyrNCACsNQ5X64P2k4Kdiyikfs6FD1atsnV");

/// vault address account for the REAL_USDC farm
pub const REAL_USDC: Pubkey = static_pubkey!("654eafNDAWhaaKe9kZ6SvHQSwBvMuSAC63wPoEzxaziQ");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const REAL_USDC_INFO_ACCOUNT: Pubkey =
    static_pubkey!("rPuJpc5iaZs91JUc5isSaq9XZVxVsXbf7WYgJ9GJuDx");
/// vault lp token account for the REAL_USDC farm
pub const REAL_USDC_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("FV2tMxjtaFRrx9CWeJTJmLwx7XureFApvgjetBqyKWA5");

/// vault address account for the CHICKS_USDC farm
pub const CHICKS_USDC: Pubkey = static_pubkey!("CEXKLVkbJYt6WwseQNgkrQEpaCJVgRdgEyx7rjQwR8Vp");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const CHICKS_USDC_INFO_ACCOUNT: Pubkey =
    static_pubkey!("31NdBJ4YEnsgUjipEvmnPmTZiQMCgVwQkAhVwfMhuJgR");
/// vault lp token account for the CHICKS_USDC farm
pub const CHICKS_USDC_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("91QP2VmwSMuP7efuVAx2QtaiTeGub6M48Te6ef1KF7MF");

/// vault address account for the MEAN_RAY farm
pub const MEAN_RAY: Pubkey = static_pubkey!("3oPV92K2MSVWv3eiyxKvf6weJHiC81rUeB5tMdA9486w");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const MEAN_RAY_INFO_ACCOUNT: Pubkey =
    static_pubkey!("5vqVANrZbPgekVhHvKFYTDQ3tCCXje9VVKZ333QbQ9Mw");
/// vault lp token account for the MEAN_RAY farm
pub const MEAN_RAY_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("3HwnBeo3heyDxZPwJaypCrBkq9ahZEMbixArJjGZeLiw");

/// vault address account for the SVT_USDC farm
pub const SVT_USDC: Pubkey = static_pubkey!("7Se757k47Zsuq6RdFMtfdh5gcjsXL5R8pN8RbKViWmUn");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const SVT_USDC_INFO_ACCOUNT: Pubkey =
    static_pubkey!("2kGdLYfnxFWa2f3nfPEV9rhG4GTcGi9mbWr8YGS7y7sB");
/// vault lp token account for the SVT_USDC farm
pub const SVT_USDC_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("2seqDrs8EkhegQLdDzeddRfF66PtJzLxVWLhEbiiBGRY");

/// vault address account for the SLC_USDC farm
pub const SLC_USDC: Pubkey = static_pubkey!("BF64bDmg9rCrt2SuZJKU8ZvN9c8i27hqGpm8dfuRHVcU");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const SLC_USDC_INFO_ACCOUNT: Pubkey =
    static_pubkey!("DKpzgawESnCuKsaMTUuPXbGQvS2bm5N4LUDk6yNTNRbv");
/// vault lp token account for the SLC_USDC farm
pub const SLC_USDC_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("F5vgezVkccTzh2A4pZcijKrYH2SXWhmgBEhY2fGcjY9s");

/// vault address account for the PSY_USDC farm
pub const PSY_USDC: Pubkey = static_pubkey!("BuvvgbYEauW3hPofGRvDb3QgpFK6tW8X1K7AzW9RiiBi");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const PSY_USDC_INFO_ACCOUNT: Pubkey =
    static_pubkey!("61ESibBpmBC47JkEYeru7W4WDru21mCth951A7JbaHLU");
/// vault lp token account for the PSY_USDC farm
pub const PSY_USDC_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("GXSSLxACc8HyYhGkxuGPxYT9SW8BqGcBjbLs1Lm4MQ46");

/// vault address account for the stSOL_USDC farm
pub const stSOL_USDC: Pubkey = static_pubkey!("4RQkLEey2WG1LnCcwMzsfQLpXsZyP4un7SpezWfJ6gkL");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const stSOL_USDC_INFO_ACCOUNT: Pubkey =
    static_pubkey!("H8k4NRrYhxQMFgchkSmBHfnobjivQWBpmWfEFVZSBoyg");
/// vault lp token account for the stSOL_USDC farm
pub const stSOL_USDC_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("5qZM5dw8NGNM93SDwbRrn2KGkfAebn1xjSnM5vQ2zCD5");

/// vault address account for the stSOL_USDT farm
pub const stSOL_USDT: Pubkey = static_pubkey!("FQktyYuv7jJmiLAwA1pDZNZSyRihafTcLfYUxCXKWbJt");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const stSOL_USDT_INFO_ACCOUNT: Pubkey =
    static_pubkey!("Ao3ny6QLoW3M8Uj19nr97APjcU92LLZkMzdoyFiqiwkq");
/// vault lp token account for the stSOL_USDT farm
pub const stSOL_USDT_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("U4wtVsGSFZ8EcP42wjiJRG4gKdvSYFkqkgCGEyiJ3ss");

/// vault address account for the ETH_stSOL farm
pub const ETH_stSOL: Pubkey = static_pubkey!("FHDTRwB2tWXNieRocoXdLR3qs2AXqaRqvYXEmQnMZhi8");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const ETH_stSOL_INFO_ACCOUNT: Pubkey =
    static_pubkey!("Eso6p6Babos4L9qag9vo879gKWsvctme9GdJMS6PRXME");
/// vault lp token account for the ETH_stSOL farm
pub const ETH_stSOL_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("DQgJXGo9jpVoksx7ywSnEMgxBzgZkZJqqKMHvr8hBsZW");

/// vault address account for the BTC_stSOL farm
pub const BTC_stSOL: Pubkey = static_pubkey!("8GdBGNgJh7mpBTQnNcyPqBpVM4VtBXE1CUNCkRxF4ppV");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const BTC_stSOL_INFO_ACCOUNT: Pubkey =
    static_pubkey!("8Vps7NREVmCsHk9jsroPuViHRQUh6akc4MUmJgL2z9G");
/// vault lp token account for the BTC_stSOL farm
pub const BTC_stSOL_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("6jyvq8hurPBz4mQgjAX6zPXnQAZVxsuCSwevoTe7sKju");

/// vault address account for the ZBC_USDC farm
pub const ZBC_USDC: Pubkey = static_pubkey!("G7pfq3tyU7p3KL8fHLX6aKWoKpX7dL3usmnE5TPZMyFc");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const ZBC_USDC_INFO_ACCOUNT: Pubkey =
    static_pubkey!("7bitWhPW2Zd3MZEXcrwaA6pWYHWa5YDVt9eDGjC6b9Av");
/// vault lp token account for the ZBC_USDC farm
pub const ZBC_USDC_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("GNSXkaT1FwGe25noXSoHvZ5Gfmde5SBQvq7WYjyw2rAz");

/// vault address account for the wALEPH_USDC farm
pub const wALEPH_USDC: Pubkey = static_pubkey!("DmwGNnzFD6cYGbJdmSSFuH2XZTozGx1634APgvmkRu9Z");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const wALEPH_USDC_INFO_ACCOUNT: Pubkey =
    static_pubkey!("3guoc8LTdz12BMKuejFyFXZLicbq9cBYHu47rnqRxtPn");
/// vault lp token account for the wALEPH_USDC farm
pub const wALEPH_USDC_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("CSrv8kWcR91Wp1UUBACwY4YPCiJLVpa9UjCUfXd9zx95");

/// vault address account for the SLCL_USDC farm
pub const SLCL_USDC: Pubkey = static_pubkey!("EvPjaeBwyQS8psmCEUZeeuiZD4uPWeYHwTneWh67CZdw");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const SLCL_USDC_INFO_ACCOUNT: Pubkey =
    static_pubkey!("9t3GsF26nnHodDpxggwpEWEYixhFC9Tq2v3gDhN484bL");
/// vault lp token account for the SLCL_USDC farm
pub const SLCL_USDC_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("BdLQhBeF62pzekEfDC5D6a7UfnKydHUadnRYsTBDTHZC");

/// vault address account for the PRISM_USDC farm
pub const PRISM_USDC: Pubkey = static_pubkey!("74Bo6PW6tMdHG9M8PYbvr2p2pfe6yam6NhhficBbjiWz");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const PRISM_USDC_INFO_ACCOUNT: Pubkey =
    static_pubkey!("G1hw6UXaE8uET2Dzg7wyUHncLKTvE9qtRbT1Pkk4Xf8y");
/// vault lp token account for the PRISM_USDC farm
pub const PRISM_USDC_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("vmcy94ss7rT1k4Q3CfA7exypEtLW77x7cMkFRCgT3PJ");

/// vault address account for the HAWK_USDC farm
pub const HAWK_USDC: Pubkey = static_pubkey!("C1z4w9JZ6H5h3nvAL4am269uqk6m5heDzBjt7mbT4KZ6");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const HAWK_USDC_INFO_ACCOUNT: Pubkey =
    static_pubkey!("GeBCf45kRPv9pffjT7aXERu2dxfrZJeR6vu8E2d4HfJd");
/// vault lp token account for the HAWK_USDC farm
pub const HAWK_USDC_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("FLSCqwuNfCgKJzWGRUvWZCHnVs6N1yQVrBFUXVokqeBb");

/// vault address account for the RAY_whETH farm
pub const RAY_whETH: Pubkey = static_pubkey!("2CQqhroSWdjVL3jptbLWGPBYWFWFC5SgvtkQqo2BDkQJ");
/// if a vault has an X_OLD_INFO variable declared use that, otherwise use this
pub const RAY_whETH_INFO_ACCOUNT: Pubkey =
    static_pubkey!("AU3RAqiReS4ygDRGi4HX6EkCBhguZpQkwa1erqpn2BQe");
/// vault lp token account for the RAY_whETH farm
pub const RAY_whETH_LP_TOKEN_ACCOUNT: Pubkey =
    static_pubkey!("6DBomSzU5YKLQvA7nTvbMu1At2NKh1Z7nruoEqus7boD");
