use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use solana_program::pubkey::Pubkey;
use static_pubkey::static_pubkey;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, AnchorSerialize, AnchorDeserialize)]
pub enum AquaFarms {
    SOLUSDC = 0,
    ORCAUSDC = 1,
    USDCUSDT = 2,
    ORCASOL = 3,
    ETHUSDC = 4,
    SOLUSDT = 5,
    ETHSOL = 6,
    mSOLSOL = 7,
    ATLASUSDC = 8,
    POLISUSDC = 9,
    SOCNSOL = 10,
    SOCNUSDC = 11,
    whETHUSDC = 12,
    whETHSOL = 13,
    SAMOUSDC = 15,
    SHDWSOL = 16,
    SHDWUSDC = 17,
    BASISUSDC = 18,
    stSOLUSDC = 19,
    stSOLwUST = 20,
    GSTUSDC = 24,
    sRLYSOL = 25,
    GMTUSDC = 26,
}
impl From<u8> for AquaFarms {
    fn from(val: u8) -> AquaFarms {
        match val {
            0 => AquaFarms::SOLUSDC,
            1 => AquaFarms::ORCAUSDC,
            2 => AquaFarms::USDCUSDT,
            3 => AquaFarms::ORCASOL,
            4 => AquaFarms::ETHUSDC,
            5 => AquaFarms::SOLUSDT,
            6 => AquaFarms::ETHSOL,
            7 => AquaFarms::mSOLSOL,
            8 => AquaFarms::ATLASUSDC,
            9 => AquaFarms::POLISUSDC,
            10 => AquaFarms::SOCNSOL,
            11 => AquaFarms::SOCNUSDC,
            12 => AquaFarms::whETHUSDC,
            13 => AquaFarms::whETHSOL,
            15 => AquaFarms::SAMOUSDC,
            16 => AquaFarms::SHDWSOL,
            17 => AquaFarms::SHDWUSDC,
            18 => AquaFarms::BASISUSDC,
            19 => AquaFarms::stSOLUSDC,
            24 => AquaFarms::GSTUSDC,
            20 => AquaFarms::stSOLwUST,
            25 => AquaFarms::sRLYSOL,
            26 => AquaFarms::GMTUSDC,
            _ => panic!(":ruhroh:"),
        }
    }
}

impl From<&str> for AquaFarms {
    fn from(val: &str) -> AquaFarms {
        match val {
            "SOL-USDC" => AquaFarms::SOLUSDC,
            "ORCA-USDC" => AquaFarms::ORCAUSDC,
            "USDT-USDC" => AquaFarms::USDCUSDT,
            "ORCA-SOL" => AquaFarms::ORCASOL,
            _ => panic!(":ruhroh:"),
        }
    }
}

impl AquaFarms {
    pub fn name(&self) -> String {
        match self {
            AquaFarms::SOLUSDC => String::from("SOL-USDC"),
            AquaFarms::ORCAUSDC => String::from("ORCA-USDC"),
            AquaFarms::USDCUSDT => String::from("USDC-USDT"),
            AquaFarms::ORCASOL => String::from("ORCA-SOL"),
            AquaFarms::ETHUSDC => String::from("ETH-USDC"),
            AquaFarms::SOLUSDT => String::from("SOL-USDT"),
            AquaFarms::ETHSOL => String::from("ETH-SOL"),
            AquaFarms::mSOLSOL => String::from("mSOL-SOL"),
            AquaFarms::ATLASUSDC => String::from("ATLAS-USDC"),
            AquaFarms::POLISUSDC => String::from("POLIS-USDC"),
            AquaFarms::SOCNSOL => String::from("SOCN-SOL"),
            AquaFarms::SOCNUSDC => String::from("SOCN-USDC"),
            AquaFarms::whETHSOL => String::from("whETH-SOL"),
            AquaFarms::whETHUSDC => String::from("whETH-USDC"),
            AquaFarms::SAMOUSDC => String::from("SAMO-USDC"),
            AquaFarms::SHDWSOL => String::from("SHDW-SOL"),
            AquaFarms::SHDWUSDC => String::from("SHDW-USDC"),
            AquaFarms::BASISUSDC => String::from("BASIS-USDC"),
            AquaFarms::stSOLUSDC => String::from("stSOL-USDC"),
            AquaFarms::GSTUSDC => String::from("GST-USDC"),
            AquaFarms::stSOLwUST => String::from("stSOL-wUST"),
            AquaFarms::sRLYSOL => String::from("sRLY-SOL"),
            AquaFarms::GMTUSDC => String::from("GMT-USDC"),
            _ => panic!(":ruhroh:"),
        }
    }
    pub fn swap_account(&self) -> Pubkey {
        match self {
            AquaFarms::SOLUSDC => Pubkey::new_from_array([
                197, 35, 245, 131, 249, 110, 212, 253, 189, 138, 222, 22, 95, 141, 254, 85, 51,
                193, 15, 160, 33, 162, 42, 63, 49, 229, 74, 220, 2, 3, 22, 97,
            ]),
            AquaFarms::ORCAUSDC => Pubkey::new_from_array([
                26, 237, 168, 53, 128, 224, 107, 139, 226, 199, 179, 245, 231, 89, 10, 105, 200,
                190, 188, 55, 198, 208, 3, 218, 37, 41, 159, 156, 162, 101, 165, 159,
            ]),
            // Pubkey::from_str("2p7nYbtPBgtmY69NsE8DAW6szpRJn7tQvDnqvoEWQvjY").unwrap(),
            AquaFarms::ORCASOL => Pubkey::new_from_array([
                23, 65, 183, 210, 120, 223, 34, 172, 145, 136, 23, 26, 89, 95, 45, 192, 98, 15, 44,
                143, 110, 130, 85, 115, 26, 99, 250, 101, 98, 44, 37, 83,
            ]),
            // Pubkey::from_str("2ZnVuidTHpi5WWKUwFXauYGhvdT9jRKYv5MDahtbwtYr").unwrap(),
            AquaFarms::USDCUSDT => Pubkey::new_from_array([
                208, 6, 247, 254, 42, 137, 109, 188, 117, 103, 175, 45, 246, 116, 129, 245, 244,
                136, 96, 106, 72, 174, 146, 169, 220, 206, 184, 111, 239, 165, 12, 58,
            ]),
            //Pubkey::from_str("F13xvvx45jVGd84ynK3c8T89UejQVxjCLtmHfPmAXAHP").unwrap(),
            AquaFarms::ETHUSDC => Pubkey::new_from_array([
                218, 38, 123, 95, 114, 198, 240, 145, 158, 152, 59, 168, 131, 54, 93, 119, 62, 76,
                233, 36, 194, 215, 255, 45, 38, 253, 181, 156, 54, 49, 161, 96,
            ]),
            // Pubkey::from_str("FgZut2qVQEyPBibaTJbbX2PxaMZvT1vjDebiVaDp5BWP").unwrap(),
            AquaFarms::SOLUSDT => Pubkey::new_from_array([
                190, 200, 223, 194, 108, 199, 171, 91, 30, 201, 36, 85, 60, 214, 64, 61, 202, 232,
                201, 187, 72, 65, 212, 177, 126, 62, 4, 61, 132, 203, 13, 177,
            ]),
            // Pubkey::from_str("Dqk7mHQBx2ZWExmyrR2S8X6UG75CrbbpK2FSBZsNYsw6").unwrap(),
            AquaFarms::ETHSOL => static_pubkey!("EuK3xDa4rWuHeMQCBsHf1ETZNiEQb5C476oE9u9kp8Ji"),
            AquaFarms::mSOLSOL => static_pubkey!("9EQMEzJdE2LDAY1hw1RytpufdwAXzatYfQ3M2UuT9b88"),
            AquaFarms::ATLASUSDC => static_pubkey!("3V5sjXj1mrWjjB1Xt6Xwp554QwHE5fppGSxbk4GzAtEW"),
            AquaFarms::POLISUSDC => static_pubkey!("CdKPtCb5fBRaGFS4bJgytfReeHuFyhpe9YUyWHPnEWZG"),
            AquaFarms::SOCNUSDC => static_pubkey!("6Gh36sNXrGWYiWr999d9iZtqgnipJbWuBohyHBN1cJpS"),
            AquaFarms::SOCNSOL => static_pubkey!("2q6UMko5kTnv866W9MTeAFau94pLpsdeNjDdSYSgZUXr"),
            AquaFarms::whETHUSDC => static_pubkey!("4reGGLbesqpAeAZdAJv9hhgA2tgj45oGcyRuEvhATdMm"),
            AquaFarms::whETHSOL => static_pubkey!("FcEro2uFpHcb7Z785CBs6q12KMJqUJKa8VTXPi4TTBMf"),
            AquaFarms::SAMOUSDC => static_pubkey!("Epvp7qMYAF21VVjacdB3VfKn6nnXQSF4rGYu8sD6Bkow"),
            AquaFarms::SHDWSOL => static_pubkey!("E3fxkJGNNAWf5xXDfMdq5qofBVkQtLKxkP7gG6Up21Ts"),
            AquaFarms::SHDWUSDC => static_pubkey!("25bQ6UzZpgFgnU7MqZdqM9Axi6oJunytRL2LgXruDWZB"),
            AquaFarms::BASISUSDC => static_pubkey!("9wb29L97MmPp7Nw8oaqiAGkXceETGNQurhySiUNLv5wh"),
            AquaFarms::stSOLUSDC => static_pubkey!("EfK84vYEKT1PoTJr6fBVKFbyA7ZoftfPo2LQPAJG1exL"),
            AquaFarms::GSTUSDC => static_pubkey!("87E4KtN7F4LivKhjqXaoQAvS3a8HnM4DnMUrbMrkVvXY"),
            AquaFarms::stSOLwUST => static_pubkey!("9F3J6RY7PTkDb3SUUpg725uXyCceBGCpZrtmYGJwgMwF"),
            AquaFarms::sRLYSOL => static_pubkey!("Df7DkQRXEpPM5basbYHi45268hmR8m7YrtraPdGgj6R6"),
            AquaFarms::GMTUSDC => static_pubkey!("46GcZFgznxUf6TpoCqJqzMpgMbbJPCAwNn8GThSt9qjC"),
        }
    }
    pub fn swap_authority(&self) -> Pubkey {
        match self {
            AquaFarms::SOLUSDC => Pubkey::new_from_array([
                4, 121, 141, 206, 21, 48, 110, 72, 85, 2, 68, 138, 228, 24, 131, 27, 166, 167, 107,
                146, 149, 99, 119, 166, 235, 126, 5, 142, 94, 1, 30, 84,
            ]),
            AquaFarms::ORCAUSDC => Pubkey::new_from_array([
                39, 170, 218, 50, 155, 230, 17, 169, 214, 103, 234, 165, 245, 108, 58, 201, 1, 92,
                231, 171, 126, 106, 133, 61, 230, 218, 190, 254, 156, 229, 83, 226,
            ]),
            // Pubkey::from_str("3fr1AhdiAmWLeNrS24CMoAu9pPgbzVhwLtJ6QUPmw2ob").unwrap(),
            AquaFarms::ORCASOL => Pubkey::new_from_array([
                20, 144, 146, 138, 38, 95, 82, 203, 72, 42, 150, 232, 54, 0, 231, 149, 232, 144,
                129, 49, 10, 38, 200, 29, 119, 152, 179, 254, 125, 52, 234, 202,
            ]),
            // Pubkey::from_str("2PH1quJj9MHQXATCmNZ6qQ2gZqM8R236DpKaz99ggVpm").unwrap(),
            AquaFarms::USDCUSDT => Pubkey::new_from_array([
                38, 191, 250, 148, 15, 160, 145, 96, 150, 150, 1, 20, 60, 30, 38, 17, 245, 150,
                154, 165, 99, 31, 53, 103, 167, 216, 220, 65, 13, 42, 244, 183,
            ]),
            AquaFarms::ETHUSDC => Pubkey::new_from_array([
                53, 247, 41, 119, 160, 134, 156, 250, 157, 124, 105, 21, 0, 116, 137, 14, 49, 68,
                243, 104, 92, 133, 85, 82, 245, 77, 51, 235, 32, 40, 210, 33,
            ]),
            AquaFarms::SOLUSDT => Pubkey::new_from_array([
                27, 233, 71, 6, 161, 116, 12, 212, 226, 245, 8, 228, 98, 54, 214, 58, 177, 221,
                247, 241, 137, 199, 157, 36, 123, 200, 140, 194, 163, 240, 153, 188,
            ]),
            AquaFarms::ETHSOL => static_pubkey!("DffrDbzPiswDJaiicBBo9CjqztKgFLrqXGwNJH4XQefZ"),
            AquaFarms::mSOLSOL => static_pubkey!("6cwehd4xhKkJ2s7iGh4CaDb7KhMgqczSBnyNJieUYbHn"),
            AquaFarms::ATLASUSDC => static_pubkey!("8UYN675AJn5htWydDs724xqintBZ4XzsCWqMozUSDU8m"),
            AquaFarms::POLISUSDC => static_pubkey!("8XB9V3VuHtPBzHqvxzcmpkpaoXNXjZMD8VBHC79SxcEL"),
            AquaFarms::SOCNUSDC => static_pubkey!("GXWEpRURaQZ9E62Q23EreTUfBy4hfemXgWFUWcg7YFgv"),
            AquaFarms::SOCNSOL => static_pubkey!("Gyd77CwV23qq937x9UDa4TDkxEeQF9tp8ifotYxqW3Kd"),
            AquaFarms::whETHUSDC => static_pubkey!("8uLtzZ1iTLTCPsm3b4QttRmDXcFjhVHRuMS9VTVEwo7E"),
            AquaFarms::whETHSOL => static_pubkey!("HMxZz8fv2uR9suzAPRbJGNB3wZL1eT3eKL3cpYWUbM8K"),
            AquaFarms::SAMOUSDC => static_pubkey!("AB4rTE2JiKFhnfynUQCovbW75CUxT9LxcJX2SDTbY9gy"),
            AquaFarms::SHDWSOL => static_pubkey!("ByC5idkRdo2XdU5U6tSoSQmfq6spztUYMaSs2rrcJRPh"),
            AquaFarms::SHDWUSDC => static_pubkey!("BjnfpyU3Verx99dKcEJZpL1AqLTPrkAUcd44LpXcXVvn"),
            AquaFarms::BASISUSDC => static_pubkey!("786ezhfHqkmJUBmjrWYGpzPnVWR8zhy2V71qNws7D89z"),
            AquaFarms::stSOLUSDC => static_pubkey!("8PSN1CQxfyZ7T4sM3HM3RAgF2Y6VCf4tKSc8xY73Tnq5"),
            AquaFarms::GSTUSDC => static_pubkey!("CwwMfXPXfRT5H5JUatpBctASRGhKW2SqLWWGU3eX5Zgo"),
            AquaFarms::stSOLwUST => static_pubkey!("wJydc21tAMxYDif8uvy5rWNGWDFNZnPPmqCvegyZRod"),
            AquaFarms::sRLYSOL => static_pubkey!("9DaRQeoEx3EjXYxhpZrcJ6no3bcAkfm9toWbngcAqSCB"),
            AquaFarms::GMTUSDC => static_pubkey!("3HGGVGTXbqT49PG3L8JQYH4jCeP5CNBG6CpJniZ434an"),
        }
    }
    pub fn swap_token_a(&self) -> Pubkey {
        match self {
            AquaFarms::SOLUSDC => Pubkey::new_from_array([
                139, 51, 95, 121, 153, 53, 170, 63, 140, 205, 147, 132, 244, 86, 254, 98, 34, 162,
                111, 125, 49, 176, 65, 67, 36, 107, 146, 112, 102, 203, 171, 157,
            ]),
            AquaFarms::ORCAUSDC => Pubkey::new_from_array([
                132, 148, 230, 160, 244, 190, 250, 208, 134, 175, 167, 202, 172, 143, 17, 252, 212,
                127, 171, 15, 119, 104, 86, 1, 26, 82, 124, 140, 83, 95, 16, 134,
            ]),
            // Pubkey::from_str("9vYWHBPz817wJdQpE8u3h8UoY3sZ16ZXdCcvLB7jY4Dj").unwrap(),
            AquaFarms::ORCASOL => Pubkey::new_from_array([
                144, 110, 129, 8, 4, 153, 213, 245, 213, 112, 114, 87, 204, 0, 74, 160, 204, 101,
                44, 70, 119, 235, 199, 225, 160, 93, 78, 145, 120, 237, 249, 234,
            ]),
            // Pubkey::from_str("AioST8HKQJRqjE1mknk4Rydc8wVADhdQwRJmAAYX1T6Z").unwrap(),
            AquaFarms::USDCUSDT => Pubkey::new_from_array([
                87, 188, 11, 39, 238, 192, 250, 140, 127, 101, 53, 217, 145, 117, 248, 249, 217,
                83, 182, 59, 119, 231, 25, 80, 238, 197, 212, 41, 177, 227, 117, 89,
            ]),
            AquaFarms::ETHUSDC => Pubkey::new_from_array([
                239, 244, 246, 95, 88, 205, 148, 152, 6, 138, 151, 51, 99, 181, 169, 197, 180, 5,
                29, 7, 84, 235, 46, 93, 197, 207, 147, 30, 241, 93, 131, 188,
            ]),
            AquaFarms::SOLUSDT => Pubkey::new_from_array([
                185, 27, 238, 132, 86, 225, 206, 235, 250, 35, 22, 21, 156, 219, 148, 71, 187, 226,
                179, 93, 190, 254, 206, 188, 41, 32, 44, 111, 173, 184, 180, 224,
            ]),
            AquaFarms::ETHSOL => static_pubkey!("7F2cLdio3i6CCJaypj9VfNDPW2DwT3vkDmZJDEfmxu6A"),
            AquaFarms::mSOLSOL => static_pubkey!("6xmki5RtGNHrfhTiHFfp9k3RQ9t8qgL1cYP2YCG2h179"),
            AquaFarms::ATLASUSDC => static_pubkey!("xotXsNCx4tBhnwhrajGTaVgKq1sfuMkeYHc77ZegCqE"),
            AquaFarms::POLISUSDC => static_pubkey!("EbXNEUiKxSU1vwwhrbVNVk3qX4o1yU3p75SQUUMfc1zH"),
            AquaFarms::SOCNUSDC => static_pubkey!("7xs9QsrxQDVoWQ8LQ8VsVjfPKBrPGjvg8ZhaLnU1i2VR"),
            AquaFarms::SOCNSOL => static_pubkey!("C8DRXUqxXtUgvgBR7BPAmy6tnRJYgVjG27VU44wWDMNV"),
            AquaFarms::whETHUSDC => static_pubkey!("9KpjcpKwhoFPbixvKDfcAhBQcVXk1CSBTGsJdzojDPRv"),
            AquaFarms::whETHSOL => static_pubkey!("3uQytDKNd5H6XK8FhTei4wCUmj2eTbLTbiLAtWk2SmbA"),
            AquaFarms::SAMOUSDC => static_pubkey!("7jwHW4Lw3nVaSJXskN5pUoKU6YB9RBVfZtGBp3VbR43U"),
            AquaFarms::SHDWSOL => static_pubkey!("9LQEB2SZQJxtLQStgXVNzgWU3LVkc4szK22iDHcSr4K9"),
            AquaFarms::SHDWUSDC => static_pubkey!("8ZVaNyNZQkcMzF7esuZoRgRo7Rc9eKEN18v4zw7Ng8JZ"),
            AquaFarms::BASISUSDC => static_pubkey!("7QM71YvJm86bN9RLFoEvyDX8dBgLh2xjnabcHf4d1Q1y"),
            AquaFarms::stSOLUSDC => static_pubkey!("9SEBxqhP8sTAzmfiQfCPim1MqQXuDPb6fkGzJF7Z339i"),
            AquaFarms::GSTUSDC => static_pubkey!("9r39vqrJuubgafaJ5aQyDWYAUQVJeyZyveBXeRqp7xev"),
            AquaFarms::stSOLwUST => static_pubkey!("GFso9SAGakm8ZFa3rmuonuerbcQ8ZbACNZN7idkKR5nw"),
            AquaFarms::sRLYSOL => static_pubkey!("AJzDsY4wnv8nWSWoBimY6hWJpWC54oEgmfbV7YGXsLww"),
            AquaFarms::GMTUSDC => static_pubkey!("BTpvbpTArnekGgbXRqjfSvp7gENtHXvZCAwuUKQNYMeN"),
        }
    }
    pub fn swap_token_b(&self) -> Pubkey {
        match self {
            AquaFarms::SOLUSDC => Pubkey::new_from_array([
                90, 63, 79, 198, 224, 89, 203, 239, 67, 163, 24, 132, 218, 145, 68, 251, 153, 234,
                3, 178, 83, 197, 191, 21, 232, 183, 3, 205, 13, 108, 180, 164,
            ]),
            AquaFarms::ORCAUSDC => Pubkey::new_from_array([
                81, 93, 208, 163, 106, 26, 173, 31, 17, 125, 169, 209, 59, 142, 95, 178, 44, 253,
                234, 141, 7, 32, 157, 102, 9, 244, 139, 7, 34, 70, 5, 100,
            ]),
            // Pubkey::from_str("6UczejMUv1tzdvUzKpULKHxrK9sqLm8edR1v9jinVWm9").unwrap(),
            AquaFarms::ORCASOL => Pubkey::new_from_array([
                89, 234, 116, 55, 49, 199, 223, 131, 195, 63, 179, 52, 25, 176, 170, 190, 65, 30,
                88, 128, 10, 221, 153, 234, 205, 18, 239, 254, 209, 10, 203, 147,
            ]),
            // Pubkey::from_str("73zdy95DynZP4exdpuXTDsexcrWbDJX9TFi2E6CDzXh4").unwrap(),
            AquaFarms::USDCUSDT => Pubkey::new_from_array([
                144, 119, 237, 48, 108, 17, 99, 104, 159, 141, 52, 92, 230, 63, 0, 12, 51, 166, 61,
                228, 47, 58, 210, 239, 250, 210, 79, 241, 121, 142, 196, 105,
            ]),
            AquaFarms::ETHUSDC => Pubkey::new_from_array([
                254, 238, 24, 106, 21, 223, 230, 121, 138, 86, 165, 142, 212, 3, 243, 121, 45, 237,
                102, 136, 106, 8, 122, 100, 86, 73, 75, 249, 236, 228, 167, 119,
            ]),
            AquaFarms::SOLUSDT => Pubkey::new_from_array([
                195, 29, 203, 108, 211, 148, 157, 207, 143, 153, 247, 11, 131, 20, 26, 188, 32,
                119, 33, 52, 209, 116, 95, 63, 13, 151, 92, 46, 131, 154, 196, 79,
            ]),
            AquaFarms::ETHSOL => static_pubkey!("5pUTGvN2AA2BEzBDU4CNDh3LHER15WS6J8oJf5XeZFD8"),
            AquaFarms::mSOLSOL => static_pubkey!("Ew2coQtVGLeca31vqB2ssHntjzZgUy1ad9VuuAX8yw7p"),
            AquaFarms::ATLASUSDC => static_pubkey!("8YswVYsTi66umBF2Bnkh4LB2VWMKPssDpe54VAgiuJZQ"),
            AquaFarms::POLISUSDC => static_pubkey!("CLCj9b1vdPutrkvZS8ACTM5q42SXB2Q7khnMLVxDMGEK"),
            AquaFarms::SOCNUSDC => static_pubkey!("FZFJK64Fk1t619zmVPqCx8Uy29zJ3WuvjWitCQuxXRo3"),
            AquaFarms::SOCNSOL => static_pubkey!("DzdxH5qJ68PiM1p5o6PbPLPpDj8m1ZshcaMFATcxDZix"),
            AquaFarms::whETHUSDC => static_pubkey!("5HaG31FQS4McBVcHxVfwaKaWXE3VCGqvJ1ZDkTxs94cQ"),
            AquaFarms::whETHSOL => static_pubkey!("GR3g8Wej3jmv92hYM1t22kaXog2xjkGjQ7V1XzLd1efT"),
            AquaFarms::SAMOUSDC => static_pubkey!("G7Gqjxk9EaJMeFfoFTSy9WfH8uurgQkbNQCREWAc56DZ"),
            AquaFarms::SHDWSOL => static_pubkey!("F2qtMkEy3L78wpw64bckvRx5M4w12Zi7bimuPBnYzto1"),
            AquaFarms::SHDWUSDC => static_pubkey!("H8A2xivBXr1RMCYmuhJ7dyEXJqPxaGDyQaaim8WucU7c"),
            AquaFarms::BASISUSDC => static_pubkey!("AmP22dYrTsG2LrkQX9cLg79jUrzDJcmWqGEWmM3Mdn46"),
            AquaFarms::stSOLUSDC => static_pubkey!("G45yhM5mZ5RXZpLxGWLk3PVzdAp33z8aH6F9mLW8fQj3"),
            AquaFarms::GSTUSDC => static_pubkey!("7LFnr5YgUyEgPMCLGNQ9N7wM5MFRNqCuRawLZTe5q4c7"),
            AquaFarms::stSOLwUST => static_pubkey!("EZ7pJskN2a4pDknrdkLzGDHpzjbfgdBj3Tt594K9HZbL"),
            AquaFarms::sRLYSOL => static_pubkey!("qytd7KfK3pFVWog53xUVE8dqD1sBxa1H13VnF6ADGSd"),
            AquaFarms::GMTUSDC => static_pubkey!("DdBTJuiAXQQ7gLVXBXNPbVEG8g1avRxiJXhH5LhBytYW"),
        }
    }
    pub fn swap_pool_mint(&self) -> Pubkey {
        match self {
            AquaFarms::SOLUSDC => Pubkey::new_from_array([
                139, 105, 207, 71, 241, 123, 106, 179, 110, 185, 220, 172, 148, 123, 186, 246, 102,
                110, 140, 43, 89, 123, 138, 37, 134, 27, 171, 253, 98, 22, 206, 42,
            ]),
            // Pubkey::from_str("APDFRM3HMr8CAGXwKHiu2f5ePSpaiEJhaURwhsRrUUt9").unwrap(),
            AquaFarms::ORCAUSDC => Pubkey::new_from_array([
                11, 143, 117, 19, 155, 183, 119, 94, 211, 201, 96, 224, 104, 190, 138, 58, 11, 4,
                84, 222, 27, 233, 168, 66, 88, 64, 79, 200, 204, 180, 194, 103,
            ]),
            // Pubkey::from_str("n8Mpu28RjeYD7oUX3LG1tPxzhRZh3YYLRSHcHRdS3Zx").unwrap(),
            AquaFarms::ORCASOL => Pubkey::new_from_array([
                28, 78, 94, 237, 160, 232, 139, 114, 167, 59, 33, 30, 187, 146, 83, 105, 11, 59,
                247, 196, 113, 241, 8, 36, 161, 114, 172, 24, 59, 146, 87, 98,
            ]),
            // Pubkey::from_str("2uVjAuRXavpM6h1scGQaxqb6HVaNRn6T2X7HHXTabz25").unwrap(),
            AquaFarms::USDCUSDT => Pubkey::new_from_array([
                238, 56, 126, 41, 194, 134, 139, 199, 160, 161, 194, 199, 158, 66, 249, 101, 211,
                102, 204, 189, 71, 160, 50, 59, 80, 61, 9, 153, 132, 244, 107, 174,
            ]),
            AquaFarms::ETHUSDC => Pubkey::new_from_array([
                39, 50, 110, 78, 3, 115, 140, 66, 221, 205, 235, 26, 163, 200, 185, 50, 101, 59,
                114, 224, 27, 27, 199, 198, 113, 69, 125, 22, 52, 217, 84, 108,
            ]),
            // Pubkey::from_str("3e1W6Aqcbuk2DfHUwRiRcyzpyYRRjg6yhZZcyEARydUX").unwrap(),
            AquaFarms::SOLUSDT => Pubkey::new_from_array([
                216, 112, 168, 5, 204, 175, 237, 125, 245, 160, 229, 42, 240, 204, 101, 197, 114,
                17, 24, 161, 232, 91, 70, 148, 218, 212, 254, 123, 252, 201, 164, 155,
            ]),
            // Pubkey::from_str("FZthQCuYHhcfiDma7QrX7buDHwrZEd7vL8SjS6LQa3Tx").unwrap(),
            AquaFarms::ETHSOL => static_pubkey!("71FymgN2ZUf7VvVTLE8jYEnjP3jSK1Frp2XT1nHs8Hob"),
            AquaFarms::mSOLSOL => static_pubkey!("29cdoMgu6MS2VXpcMo1sqRdWEzdUR9tjvoh8fcK8Z87R"),
            AquaFarms::ATLASUSDC => static_pubkey!("FZ8x1LCRSPDeHBDoAc3Gc6Y7ETCynuHEr5q5YWV7uRCJ"),
            AquaFarms::POLISUSDC => static_pubkey!("GteBdo9sqE7T41G8AJsaG9WHW48uXBwsLLznmu2TBdgy"),
            AquaFarms::SOCNUSDC => static_pubkey!("Dkr8B675PGnNwEr9vTKXznjjHke5454EQdz3iaSbparB"),
            AquaFarms::SOCNSOL => static_pubkey!("APNpzQvR91v1THbsAyG3HHrUEwvexWYeNCFLQuVnxgMc"),
            AquaFarms::whETHUSDC => static_pubkey!("7NPtjjAP7vhp4t5NCLyY4DY5rurvyc8cgZ2a2rYabRia"),
            AquaFarms::whETHSOL => static_pubkey!("7aYnrdmdCRodDy2Czn6keUquUhjF1jPEmfwZPh488z8U"),
            AquaFarms::SAMOUSDC => static_pubkey!("6VK1ksrmYGMBWUUZfygGF8tHRGpNxQEWv8pfvzQHdyyc"),
            AquaFarms::SHDWSOL => static_pubkey!("2ws7g3LBPdctfKn42Di9qxzQtUJ8ZL1aEAX2rGEQMNqh"),
            AquaFarms::SHDWUSDC => static_pubkey!("DJqqvzSuPaWThfzwMjXx7H2ZmHDdwxza6NtFudtuXcpc"),
            AquaFarms::BASISUSDC => static_pubkey!("GoaAiajubRgeCFEz9L6mLnSmT2QFegoJDH5tpLfivpj"),
            AquaFarms::stSOLUSDC => static_pubkey!("GtQ1NT7R5aaTiST7K6ZWdMhwDdFxsSFvVFhBo8vyHGAq"),
            AquaFarms::GSTUSDC => static_pubkey!("E6FUnQHGHJVJg7oExVr5Moeaj1QpdpZQF5odYjHXWPZb"),
            AquaFarms::stSOLwUST => static_pubkey!("HTZd53fYwYQRyAjiaPsZy9Gf41gobFdqkF4oKe3XLi95"),
            AquaFarms::sRLYSOL => static_pubkey!("3dXdXg5HPyZ73GFC9LkSn3thdJUGeXWB8iSTHs5UcqiH"),
            AquaFarms::GMTUSDC => static_pubkey!("CFxQF5kNAtbbDj298Xr47Sf4mkSyuzWpRH97hrdQ6kxi"),
        }
    }
    // this seems to not either be used, or ends up being the swap_token_a :think:
    pub fn emissions_authority(&self) -> Pubkey {
        match self {
            AquaFarms::SOLUSDC => Pubkey::new_from_array([
                139, 51, 95, 121, 153, 53, 170, 63, 140, 205, 147, 132, 244, 86, 254, 98, 34, 162,
                111, 125, 49, 176, 65, 67, 36, 107, 146, 112, 102, 203, 171, 157,
            ]),
            // Pubkey::from_str("ANP74VNsHwSrq9uUSjiSNyNWvf6ZPrKTmE4gHoNd13Lg").unwrap(),
            AquaFarms::ORCAUSDC => Pubkey::new_from_array([
                132, 148, 230, 160, 244, 190, 250, 208, 134, 175, 167, 202, 172, 143, 17, 252, 212,
                127, 171, 15, 119, 104, 86, 1, 26, 82, 124, 140, 83, 95, 16, 134,
            ]),
            // Pubkey::from_str("9vYWHBPz817wJdQpE8u3h8UoY3sZ16ZXdCcvLB7jY4Dj").unwrap(),
            AquaFarms::USDCUSDT => Pubkey::new_from_array([
                87, 188, 11, 39, 238, 192, 250, 140, 127, 101, 53, 217, 145, 117, 248, 249, 217,
                83, 182, 59, 119, 231, 25, 80, 238, 197, 212, 41, 177, 227, 117, 89,
            ]),
            AquaFarms::ORCASOL => Pubkey::new_from_array([
                82, 116, 173, 238, 105, 141, 1, 73, 56, 85, 4, 27, 234, 46, 194, 63, 15, 193, 179,
                238, 53, 58, 158, 7, 216, 252, 154, 33, 201, 25, 50, 166,
            ]),
            _ => {
                println!("not used");
                Pubkey::default()
            }
        }
    }
    pub fn remove_rewards_authority(&self) -> Pubkey {
        match self {
            AquaFarms::SOLUSDC => Pubkey::new_from_array([
                90, 63, 79, 198, 224, 89, 203, 239, 67, 163, 24, 132, 218, 145, 68, 251, 153, 234,
                3, 178, 83, 197, 191, 21, 232, 183, 3, 205, 13, 108, 180, 164,
            ]),
            // Pubkey::from_str("75HgnSvXbWKZBpZHveX68ZzAhDqMzNDS29X6BGLtxMo1").unwrap(),
            AquaFarms::ORCAUSDC => Pubkey::new_from_array([
                81, 93, 208, 163, 106, 26, 173, 31, 17, 125, 169, 209, 59, 142, 95, 178, 44, 253,
                234, 141, 7, 32, 157, 102, 9, 244, 139, 7, 34, 70, 5, 100,
            ]),
            // Pubkey::from_str("6UczejMUv1tzdvUzKpULKHxrK9sqLm8edR1v9jinVWm9").unwrap(),
            AquaFarms::USDCUSDT => Pubkey::new_from_array([
                144, 119, 237, 48, 108, 17, 99, 104, 159, 141, 52, 92, 230, 63, 0, 12, 51, 166, 61,
                228, 47, 58, 210, 239, 250, 210, 79, 241, 121, 142, 196, 105,
            ]),
            AquaFarms::ORCASOL => Pubkey::new_from_array([
                82, 116, 173, 238, 105, 141, 1, 73, 56, 85, 4, 27, 234, 46, 194, 63, 15, 193, 179,
                238, 53, 58, 158, 7, 216, 252, 154, 33, 201, 25, 50, 166,
            ]),
            // Pubkey::from_str("6Ysd94bSnDknaaUitt3FyKia7bDcPjL7MseXDHSCKipV").unwrap(),
            // Pubkey::from_str("AiwmnLy7xPT28dqZpkRm6i1ZGwELUCzCsuN92v4JkSeU").unwrap(),
            _ => {
                println!("not used");
                Pubkey::default()
            }
        }
    }
    pub fn base_token_mint(&self) -> Pubkey {
        match self {
            AquaFarms::SOLUSDC | AquaFarms::SOLUSDT => Pubkey::new_from_array([
                6, 155, 136, 87, 254, 171, 129, 132, 251, 104, 127, 99, 70, 24, 192, 53, 218, 196,
                57, 220, 26, 235, 59, 85, 152, 160, 240, 0, 0, 0, 0, 1,
            ]),
            // Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap(),
            AquaFarms::ORCAUSDC | AquaFarms::ORCASOL => Pubkey::new_from_array([
                12, 0, 208, 175, 235, 134, 20, 218, 127, 25, 171, 160, 45, 64, 241, 140, 105, 37,
                133, 246, 80, 32, 223, 206, 211, 213, 229, 249, 169, 192, 196, 225,
            ]),
            // Pubkey::from_str("orcaEKTdK7LKz57vaAYr9QeNsVEPfiu6QeMU1kektZE").unwrap(),
            AquaFarms::USDCUSDT => Pubkey::new_from_array([
                198, 250, 122, 243, 190, 219, 173, 58, 61, 101, 243, 106, 171, 201, 116, 49, 177,
                187, 228, 194, 210, 246, 224, 228, 124, 166, 2, 3, 69, 47, 93, 97,
            ]),
            // Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").unwrap(),
            AquaFarms::ETHUSDC | AquaFarms::ETHSOL => Pubkey::new_from_array([
                18, 139, 203, 100, 125, 139, 173, 30, 114, 80, 227, 184, 52, 188, 250, 159, 217,
                134, 244, 212, 119, 209, 187, 185, 5, 78, 96, 43, 17, 235, 224, 97,
            ]),
            // Pubkey::from_str("2FPyTwcZLUg1MDrwsyoP4D6s1tM7hAkHYRjkNb5w6Pxk").unwrap(),
            AquaFarms::mSOLSOL => static_pubkey!("mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So"),
            AquaFarms::ATLASUSDC => static_pubkey!("ATLASXmbPQxBUYbxPsV97usA3fPQYEqzQBUHgiFCUsXx"),
            AquaFarms::POLISUSDC => static_pubkey!("poLisWXnNRwC6oBu1vHiuKQzFjGL4XDSu4g9qjz9qVk"),
            AquaFarms::SOCNSOL | AquaFarms::SOCNUSDC => {
                static_pubkey!("5oVNBeEEQvYi1cX3ir8Dx5n1P7pdxydbGF2X4TxVusJm")
            }
            AquaFarms::whETHUSDC | AquaFarms::whETHSOL => {
                static_pubkey!("7vfCXTUXx5WJV5JADk17DUJ4ksgau7utNKj4b963voxs")
            }
            AquaFarms::SAMOUSDC => static_pubkey!("7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU"),
            AquaFarms::SHDWUSDC | AquaFarms::SHDWSOL => {
                static_pubkey!("SHDWyBxihqiCj6YekG2GUr7wqKLeLAMK1gHZck9pL6y")
            }
            AquaFarms::BASISUSDC => static_pubkey!("Basis9oJw9j8cw53oMV7iqsgo6ihi9ALw4QR31rcjUJa"),
            AquaFarms::stSOLUSDC => static_pubkey!("7dHbWXmci3dT8UFYWYZweBLXgycu7Y3iL6trKn1Y7ARj"),
            AquaFarms::GSTUSDC => static_pubkey!("AFbX8oGjGpmVFywbVouvhQSRmiW2aR1mohfahi4Y2AdB"),
            AquaFarms::stSOLUSDC | AquaFarms::stSOLwUST => {
                static_pubkey!("7dHbWXmci3dT8UFYWYZweBLXgycu7Y3iL6trKn1Y7ARj")
            }
            AquaFarms::sRLYSOL => static_pubkey!("RLYv2ubRMDLcGG2UyvPmnPmkfuQTsMbg4Jtygc7dmnq"),
            AquaFarms::GMTUSDC => static_pubkey!("7i5KKsX2weiTkry7jA4ZwSuXGhs5eJBEjY8vVxR4pfRx"),
        }
    }
    pub fn quote_token_mint(&self) -> Pubkey {
        match self {
            AquaFarms::ORCAUSDC
            | AquaFarms::SOLUSDC
            | AquaFarms::ATLASUSDC
            | AquaFarms::POLISUSDC
            | AquaFarms::SOCNUSDC
            | AquaFarms::whETHUSDC
            | AquaFarms::SAMOUSDC
            | AquaFarms::SHDWUSDC
            | AquaFarms::BASISUSDC
            | AquaFarms::stSOLUSDC
            | AquaFarms::GSTUSDC
            | AquaFarms::GMTUSDC => Pubkey::new_from_array([
                198, 250, 122, 243, 190, 219, 173, 58, 61, 101, 243, 106, 171, 201, 116, 49, 177,
                187, 228, 194, 210, 246, 224, 228, 124, 166, 2, 3, 69, 47, 93, 97,
            ]),
            AquaFarms::ORCASOL
            | AquaFarms::ETHSOL
            | AquaFarms::mSOLSOL
            | AquaFarms::SOCNSOL
            | AquaFarms::whETHSOL
            | AquaFarms::SHDWSOL
            | AquaFarms::sRLYSOL => Pubkey::new_from_array([
                6, 155, 136, 87, 254, 171, 129, 132, 251, 104, 127, 99, 70, 24, 192, 53, 218, 196,
                57, 220, 26, 235, 59, 85, 152, 160, 240, 0, 0, 0, 0, 1,
            ]),
            AquaFarms::ETHUSDC => Pubkey::new_from_array([
                198, 250, 122, 243, 190, 219, 173, 58, 61, 101, 243, 106, 171, 201, 116, 49, 177,
                187, 228, 194, 210, 246, 224, 228, 124, 166, 2, 3, 69, 47, 93, 97,
            ]),
            AquaFarms::SOLUSDT | AquaFarms::USDCUSDT => Pubkey::new_from_array([
                206, 1, 14, 96, 175, 237, 178, 39, 23, 189, 99, 25, 47, 84, 20, 90, 63, 150, 90,
                51, 187, 130, 210, 199, 2, 158, 178, 206, 30, 32, 130, 100,
            ]),
            AquaFarms::stSOLwUST => static_pubkey!("9vMJfxuKxXBoEa7rM12mYLMwTacLMLDJqHozw96WQL8i"),
            // Pubkey::from_str("Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB").unwrap(),
        }
    }
    pub fn base_token_vault(&self) -> Pubkey {
        match self {
            AquaFarms::SOLUSDC => Pubkey::new_from_array([
                139, 105, 207, 71, 241, 123, 106, 179, 110, 185, 220, 172, 148, 123, 186, 246, 102,
                110, 140, 43, 89, 123, 138, 37, 134, 27, 171, 253, 98, 22, 206, 42,
            ]),
            // Pubkey::from_str("APDFRM3HMr8CAGXwKHiu2f5ePSpaiEJhaURwhsRrUUt9").unwrap(),
            AquaFarms::ORCAUSDC => Pubkey::new_from_array([
                11, 143, 117, 19, 155, 183, 119, 94, 211, 201, 96, 224, 104, 190, 138, 58, 11, 4,
                84, 222, 27, 233, 168, 66, 88, 64, 79, 200, 204, 180, 194, 103,
            ]),
            // Pubkey::from_str("n8Mpu28RjeYD7oUX3LG1tPxzhRZh3YYLRSHcHRdS3Zx").unwrap(),
            AquaFarms::USDCUSDT => Pubkey::new_from_array([
                238, 56, 126, 41, 194, 134, 139, 199, 160, 161, 194, 199, 158, 66, 249, 101, 211,
                102, 204, 189, 71, 160, 50, 59, 80, 61, 9, 153, 132, 244, 107, 174,
            ]),
            AquaFarms::ORCASOL => Pubkey::new_from_array([
                94, 143, 59, 148, 106, 39, 22, 53, 124, 200, 138, 224, 15, 10, 251, 94, 14, 228, 3,
                180, 249, 62, 152, 67, 65, 50, 189, 99, 166, 201, 59, 28,
            ]),
            // Pubkey::from_str("H2uzgruPvonVpCRhwwdukcpXK8TG17swFNzYFr2rtPxy").unwrap(),
            AquaFarms::ETHUSDC => Pubkey::new_from_array([
                89, 25, 42, 160, 45, 55, 7, 61, 214, 127, 58, 104, 202, 13, 229, 114, 205, 102,
                248, 69, 77, 113, 91, 253, 47, 115, 151, 72, 113, 173, 47, 94,
            ]),
            // Pubkey::from_str("6zoYTvgLd4UAhKSPwirEU9VNNNkpezwq8AM4jXW1Qop9").unwrap(),
            AquaFarms::SOLUSDT => Pubkey::new_from_array([
                201, 21, 219, 72, 97, 50, 167, 150, 96, 160, 11, 119, 26, 48, 107, 184, 244, 58,
                95, 120, 114, 186, 139, 236, 252, 79, 234, 190, 202, 81, 42, 74,
            ]),
            // Pubkey::from_str("EXxH5tKDHLy68nWXS8w1BRUsiDEHMbKACLUmFWv8Q9tu").unwrap(),
            AquaFarms::ETHSOL => static_pubkey!("6ckhPnn6tCr88aq9SxhWaAA5G7izuXNKhVk1Xa62zhFD"),
            AquaFarms::mSOLSOL => static_pubkey!("DuTZUmTRydVc3EN78brdYFUfskn6s93zH4WhY3Fo53AJ"),
            AquaFarms::ATLASUSDC => static_pubkey!("AuZoiJ3Y4P55YrT7bZnhE6wCE5BVAn7rEYHjb7rHvFMF"),
            AquaFarms::POLISUSDC => static_pubkey!("hHJ1H8aj2LoRTPXGVXda2ZWHbcDwyZew9dUMdD2zKAJ"),
            AquaFarms::SOCNUSDC => static_pubkey!("GsjLcoyQEmxE2dgvv61dGqWV6AanY4goUHpax8YbyKcE"),
            AquaFarms::SOCNSOL => static_pubkey!("BAzAae9VKYg1VrJ4sDimdydpeiDdinJiFopwsgAxPrhJ"),
            AquaFarms::whETHUSDC => static_pubkey!("FAgiNoPVnBDXaAytHwoZ36xRMt7h1QZiAmT6ikD5dA98"),
            AquaFarms::whETHSOL => static_pubkey!("H4PuQ5Ly11bYYHVyZ6dSFKARheKaryYyz991ctkFxBXU"),
            AquaFarms::SAMOUSDC => static_pubkey!("TuEvcp71fWQbRXZjW1JEKkYrNpAS4EpgibMvNeYar9j"),
            AquaFarms::SHDWSOL => static_pubkey!("GL62E2J5syXiNpude4vLiBeivvSiVTNwPRyYXWnV32d1"),
            AquaFarms::SHDWUSDC => static_pubkey!("HgM3TutTswwM3LV8LiRj1uVjK8NfeTgq9aagfLLjrY3g"),
            AquaFarms::BASISUSDC => static_pubkey!("GtRcfta8aD8BbZqAZV2gaWELSZBe4qKvexTWqSRUvdYw"),
            AquaFarms::stSOLUSDC => static_pubkey!("8iyvVva8sg79HdLSokCLFF3M1sAmagByTSz4XeQHQ6Vk"),
            AquaFarms::GSTUSDC => static_pubkey!("9swuhhrnLGDzXKAicByxMYBnSANJojeaaCJ3uENWuijq"),
            AquaFarms::stSOLwUST => static_pubkey!("EQBy5YqWkLxr1xx1CZh2dDdX57XR9Ata4jNJebtuR61h"),
            AquaFarms::sRLYSOL => static_pubkey!("8phz73Nyk4XKk7btjEJbEVyvFkcXQUvVEsYcjvoxwrBf"),
            AquaFarms::GMTUSDC => static_pubkey!("FPi9U6fLhVBggWdTwsPgByjEps6w8DiMyU5LqjLtvXnd"),
        }
    }
    pub fn reward_token_vault(&self) -> Pubkey {
        match self {
            AquaFarms::SOLUSDC => Pubkey::new_from_array([
                198, 250, 122, 243, 190, 219, 173, 58, 61, 101, 243, 106, 171, 201, 116, 49, 177,
                187, 228, 194, 210, 246, 224, 228, 124, 166, 2, 3, 69, 47, 93, 97,
            ]),
            // Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").unwrap(),
            AquaFarms::ORCAUSDC => Pubkey::new_from_array([
                198, 250, 122, 243, 190, 219, 173, 58, 61, 101, 243, 106, 171, 201, 116, 49, 177,
                187, 228, 194, 210, 246, 224, 228, 124, 166, 2, 3, 69, 47, 93, 97,
            ]),
            // Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").unwrap(),
            AquaFarms::USDCUSDT => Pubkey::new_from_array([
                206, 1, 14, 96, 175, 237, 178, 39, 23, 189, 99, 25, 47, 84, 20, 90, 63, 150, 90,
                51, 187, 130, 210, 199, 2, 158, 178, 206, 30, 32, 130, 100,
            ]),
            AquaFarms::ORCASOL => Pubkey::new_from_array([
                169, 255, 32, 179, 177, 99, 192, 192, 32, 132, 52, 115, 105, 234, 70, 28, 225, 177,
                201, 238, 182, 36, 231, 174, 189, 142, 187, 35, 139, 198, 39, 114,
            ]),
            // Pubkey::from_str("CSbYA7Cd65Vis2oqX797zmnWmpgENmqrPdmPbTbRPykd").unwrap(),
            // Pubkey::from_str("Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB").unwrap(),
            AquaFarms::ETHUSDC => Pubkey::new_from_array([
                124, 30, 63, 149, 46, 134, 4, 131, 176, 54, 212, 216, 103, 124, 157, 45, 11, 188,
                116, 1, 156, 93, 215, 183, 87, 82, 111, 110, 233, 134, 213, 180,
            ]),
            // Pubkey::from_str("9MWJmWVAGQ9C9SxwWKidStAA8HjDHpnZ7KfKgVJdrNtj").unwrap(),
            AquaFarms::SOLUSDT => Pubkey::new_from_array([
                238, 115, 75, 60, 198, 23, 230, 178, 209, 70, 119, 252, 220, 217, 26, 47, 169, 15,
                253, 114, 251, 105, 226, 127, 81, 235, 67, 113, 112, 70, 174, 93,
            ]),
            // Pubkey::from_str("H3ozvCeEwnsqnM2naCnXVxLLwH2XPC5kU8BH97XDpDwS").unwrap(),
            AquaFarms::ETHSOL => static_pubkey!("FYTTVMqWPzbnhTsukgiWmPiNJam4yLTxHM9mpzdan2zo"),
            AquaFarms::mSOLSOL => static_pubkey!("7dpUACKvEiuq5kyoGtgiA131hYwdxfFhEeD5TMT4mnzG"),
            AquaFarms::ATLASUSDC => static_pubkey!("7AxW2s5dqKphs9pjVzwNsVHqSznEyLkrtu3w61TQNvHd"),
            AquaFarms::POLISUSDC => static_pubkey!("BsgUyVxojJCtA11JxJ8R7HorsdKogGf8U66rUouZkc7X"),
            AquaFarms::SOCNUSDC => static_pubkey!("5W59sjf3jXgA6JEuUgrTZPdU3zEJ8WvXB21uNMY3aWGb"),
            AquaFarms::SOCNSOL => static_pubkey!("AUKieh8bcCMYttasy34ft5KoZ7SbQepk83KE3eeoKJv5"),
            AquaFarms::whETHUSDC => static_pubkey!("EAVw4je8KWozucNKMrJDnN1xX6Fv4CpmYsG23N3c3Kjz"),
            AquaFarms::whETHSOL => static_pubkey!("2kdi5HdZAdF7mfzaZtbog5Jo2VvzZe8V483qTCwGae4Z"),
            AquaFarms::SAMOUSDC => static_pubkey!("F8vsyagqHVJPL6whBs5BCTdfAcESRBGfGZbjKtciYzUM"),
            AquaFarms::SHDWSOL => static_pubkey!("5tdGnUMzSKiBmK9K8MpeNFYZimZNKoVFga8vm4iAMQnt"),
            AquaFarms::SHDWUSDC => static_pubkey!("HP6oppmHZr3erGPEGcotgcx8y4PPvdZYcC8PE8pU2htu"),
            AquaFarms::BASISUSDC => static_pubkey!("2CAFkxSUTSUhVDncagnRXooi6wfeakVhc3WB4bEZF8K7"),
            AquaFarms::stSOLUSDC => static_pubkey!("12QCfo5SdFuyC9P5oioken91gCYK5vnRbXaDVuV5KLpZ"),
            AquaFarms::GSTUSDC => static_pubkey!("9MboiCmp7Gas6nRX39oRcFge66Efkn5Ui9fz7SrQmue9"),
            AquaFarms::stSOLwUST => static_pubkey!("FFjuVr4FYdRGkPwaQREVQWHsBFGwQ1LH6VkmVSJL5Vva"),
            AquaFarms::sRLYSOL => static_pubkey!("9WQhuAMn99aCkZTrREjSzogrceuPrrtfXxmjKymgqNzt"),
            AquaFarms::GMTUSDC => static_pubkey!("9rc6Cyw21Kx8ofamntNVWa9v8vza7U4hGwxne764qL25"),
        }
    }
    pub fn global_farm(&self) -> Pubkey {
        match self {
            AquaFarms::SOLUSDC => Pubkey::new_from_array([
                105, 27, 47, 145, 255, 11, 24, 17, 157, 159, 239, 250, 79, 160, 200, 249, 12, 46,
                26, 204, 180, 134, 154, 157, 107, 178, 254, 1, 255, 157, 90, 159,
            ]),
            // Pubkey::from_str("85HrPbJtrN82aeB74WTwoFxcNgmf5aDNP2ENngbDpd5G").unwrap(),
            AquaFarms::ORCAUSDC => Pubkey::new_from_array([
                125, 69, 58, 75, 130, 111, 1, 173, 161, 77, 9, 72, 70, 25, 179, 140, 13, 98, 50,
                231, 29, 140, 17, 78, 29, 51, 48, 138, 184, 250, 67, 92,
            ]),
            // Pubkey::from_str("9S1BsxbDNQXQccjFamVEGgxiYQHTeudvhEYwFr4oWeaf").unwrap(),
            AquaFarms::USDCUSDT => Pubkey::new_from_array([
                71, 177, 240, 21, 55, 19, 96, 139, 133, 195, 205, 25, 202, 247, 69, 244, 143, 181,
                165, 77, 108, 66, 198, 69, 2, 143, 64, 99, 209, 52, 44, 150,
            ]),
            AquaFarms::ORCASOL => Pubkey::new_from_array([
                209, 129, 119, 45, 185, 118, 140, 138, 183, 115, 171, 19, 170, 85, 230, 225, 235,
                69, 197, 87, 244, 37, 122, 153, 90, 67, 51, 195, 113, 211, 221, 114,
            ]),
            // Pubkey::from_str("F6pi7SyXWx56fP96mYQ4Yfh4yZ7oGNtDjwSYHT5Mz7Ld").unwrap(),
            // Pubkey::from_str("5psKJrxWnPmoAbCxk3An2CGh7wHAX2cWddf5vZuYbbVw").unwrap(),
            AquaFarms::ETHUSDC => Pubkey::new_from_array([
                220, 56, 223, 1, 122, 219, 90, 28, 141, 177, 31, 38, 114, 66, 190, 144, 223, 152,
                168, 206, 37, 244, 169, 61, 50, 39, 140, 244, 51, 188, 199, 204,
            ]),
            // Pubkey::from_str("FpezTR76RRjgpBb9HhR6ap8BgQfkHyNMQSqJDcoXpjAb").unwrap(),
            AquaFarms::SOLUSDT => Pubkey::new_from_array([
                50, 212, 156, 101, 215, 175, 39, 244, 117, 225, 138, 227, 195, 196, 201, 230, 238,
                58, 222, 142, 21, 86, 239, 168, 190, 132, 237, 166, 223, 73, 12, 167,
            ]),
            // Pubkey::from_str("4RRRJkscV2DmwJUxTQgRdYock75GfwYJn7LTxy9rGTmY").unwrap(),
            AquaFarms::ETHSOL => static_pubkey!("3ARgavt1NhqLmJWj3wAJy6XBarG6pJbEKRv1wzzRbbaN"),
            AquaFarms::mSOLSOL => static_pubkey!("JADWjBW1Xs8WhW8kj3GTCRQn3LR4gwvbFTEMwv9ZNxQh"),
            AquaFarms::ATLASUSDC => static_pubkey!("G92aJeZBCFiECwrKSQsbobfykh6cNLCf5Pd3zkiLjGLe"),
            AquaFarms::POLISUSDC => static_pubkey!("5NkDw3LcFscf5WSxUiquNregP6RaP79Y2pN1jyUYepzc"),
            AquaFarms::SOCNUSDC => static_pubkey!("5MzBKRo6YqK1BKBz67sXd42jrb6gYzBuX6R5F8ywC33e"),
            AquaFarms::SOCNSOL => static_pubkey!("5cE7V9D13k1P1qC23g5vcQEMsvrDzL5yFHhiesVUyn93"),
            AquaFarms::whETHUSDC => static_pubkey!("GdQyNtN9rQWzpcm7mQMNBiXyeKRjjQoobh2waVQq5QyP"),
            AquaFarms::whETHSOL => static_pubkey!("BerS3SE5G6FqZER7L7G3BhUJBrZ7BpizmuQRH9LMEYQw"),
            AquaFarms::SAMOUSDC => static_pubkey!("F5ZvDWRVpQP5A17RFogL4dE7gZ2Uda7ZqKUy3DWJDEFx"),
            AquaFarms::SHDWSOL => static_pubkey!("EKe5CgBnBJA2cgeEUe67aQS57bAQsPvfVdArEWuEuEEW"),
            AquaFarms::SHDWUSDC => static_pubkey!("ABmFqgfvQjjU8uBkZL2KdH5AvYEMsuddtdvpm4s62Pzq"),
            AquaFarms::BASISUSDC => static_pubkey!("A8CNiARq7zYMMGKYbqJVfByVyBzdMexhc5EEGzCN13dS"),
            AquaFarms::stSOLUSDC => static_pubkey!("2P7FGV8XNXUkEAG6q5LbhfoBFkHJ7PDAjYqmAbwnVHBF"),
            AquaFarms::GSTUSDC => static_pubkey!("44aNS8nnj4r3WWvnR1Ud929iAqn5jJ7ugVkJJvH4XuW9"),
            AquaFarms::stSOLwUST => static_pubkey!("GxhewC22S6wsXT156yC9SARvDnijoc3YEYyLVcQFDUCx"),
            AquaFarms::sRLYSOL => static_pubkey!("9SV2NL59i1PfD72AUHDXa2KT1xWnFD3VyMgfzCRDARW1"),
            AquaFarms::GMTUSDC => static_pubkey!("DbJT9UTTt8U8xAk7BtRWC3nQHxJnNmZdZydUAhJj14TZ"),
        }
    }
    pub fn global_farm_dd(&self) -> Pubkey {
        match self {
            AquaFarms::ATLASUSDC => static_pubkey!("DTP1xr4EzFf1YDu4CeWTtWVsCBzFPk4HDEsL3AzoR3kB"),
            AquaFarms::POLISUSDC => static_pubkey!("7h1zAHj2xzEw3eKfprYqG36aN5XwcZXBsYwM2haWQVzR"),
            AquaFarms::SAMOUSDC => static_pubkey!("8VJT2SYGXgvQ8jYvh1Cq6mg83gkAVuTY3cHiULUy6Cit"),
            AquaFarms::SHDWSOL => static_pubkey!("2EHJ8ToKpJfXyAfechjH9QSbVMKTSViYPdJRepQz7V8S"),
            AquaFarms::SHDWUSDC => static_pubkey!("GJ9EixfM3noFT1b7Y5uAzV1qJSFev5uPaRrmfMoibck6"),
            AquaFarms::BASISUSDC => static_pubkey!("DasaXe2Wqcks6csFv1bWwdW41mV8rMD5c27Uw9rFYVu4"),
            AquaFarms::stSOLUSDC => static_pubkey!("HhBtZgPaFb5rodxXsZbtZUqxk2vEg6tdzM4AcaG2PjcL"),
            AquaFarms::stSOLwUST => static_pubkey!("CXbiLWJoYcVmV7GcF9xSwwMPSgHh5rHRLJB84F54R4qU"),
            AquaFarms::sRLYSOL => static_pubkey!("7GAp1xR64GjjbyHbvE28Rt9xLEBLTjk4W3AemRMvTcrS"),

            _ => panic!(":ruhroh:"),
        }
    }

    pub fn global_base_token_vault(&self) -> Pubkey {
        match self {
            AquaFarms::SOLUSDC => Pubkey::new_from_array([
                99, 220, 207, 233, 17, 50, 237, 91, 222, 4, 196, 130, 180, 240, 102, 119, 74, 64,
                4, 144, 196, 164, 158, 5, 93, 231, 9, 144, 165, 214, 175, 240,
            ]),
            // Pubkey::from_str("7ipefo5V3QEJWeuT2PohFSEUaranZxMSeWQo2rcNigr3").unwrap(),
            AquaFarms::ORCAUSDC => Pubkey::new_from_array([
                45, 164, 222, 162, 92, 75, 170, 210, 121, 223, 96, 177, 247, 209, 124, 98, 114, 84,
                235, 100, 180, 14, 228, 147, 162, 132, 165, 170, 236, 231, 51, 150,
            ]),
            // Pubkey::from_str("45BAAQCZYd2kP3Z3WvRwdtfUhvuW4FvpqVK4m8qrR5x1").unwrap(),
            AquaFarms::ORCASOL => Pubkey::new_from_array([
                94, 143, 59, 148, 106, 39, 22, 53, 124, 200, 138, 224, 15, 10, 251, 94, 14, 228, 3,
                180, 249, 62, 152, 67, 65, 50, 189, 99, 166, 201, 59, 28,
            ]),
            // Pubkey::from_str("7N7zxoDMMV1sCDiVEzinTyQxS2GoN388QprMCQX38BeT").unwrap(),
            AquaFarms::USDCUSDT => Pubkey::new_from_array([
                129, 54, 21, 139, 136, 112, 159, 212, 2, 28, 153, 219, 137, 7, 96, 106, 210, 191,
                92, 36, 121, 147, 33, 3, 99, 43, 89, 242, 7, 41, 146, 249,
            ]),
            // Pubkey::from_str("9hPRfmQmZYiL4ZtuvGBk5SjMzmFCQ2h9a4GKoM82BR84").unwrap(),
            AquaFarms::ETHUSDC => Pubkey::new_from_array([
                89, 25, 42, 160, 45, 55, 7, 61, 214, 127, 58, 104, 202, 13, 229, 114, 205, 102,
                248, 69, 77, 113, 91, 253, 47, 115, 151, 72, 113, 173, 47, 94,
            ]),
            // Pubkey::from_str("6zoYTvgLd4UAhKSPwirEU9VNNNkpezwq8AM4jXW1Qop9").unwrap(),
            AquaFarms::SOLUSDT => Pubkey::new_from_array([
                201, 21, 219, 72, 97, 50, 167, 150, 96, 160, 11, 119, 26, 48, 107, 184, 244, 58,
                95, 120, 114, 186, 139, 236, 252, 79, 234, 190, 202, 81, 42, 74,
            ]),
            // Pubkey::from_str("EXxH5tKDHLy68nWXS8w1BRUsiDEHMbKACLUmFWv8Q9tu").unwrap(),
            AquaFarms::ETHSOL => static_pubkey!("6ckhPnn6tCr88aq9SxhWaAA5G7izuXNKhVk1Xa62zhFD"),
            AquaFarms::mSOLSOL => static_pubkey!("DuTZUmTRydVc3EN78brdYFUfskn6s93zH4WhY3Fo53AJ"),
            AquaFarms::ATLASUSDC => static_pubkey!("AuZoiJ3Y4P55YrT7bZnhE6wCE5BVAn7rEYHjb7rHvFMF"),
            AquaFarms::POLISUSDC => static_pubkey!("hHJ1H8aj2LoRTPXGVXda2ZWHbcDwyZew9dUMdD2zKAJ"),
            AquaFarms::SOCNUSDC => static_pubkey!("GsjLcoyQEmxE2dgvv61dGqWV6AanY4goUHpax8YbyKcE"),
            AquaFarms::SOCNSOL => static_pubkey!("BAzAae9VKYg1VrJ4sDimdydpeiDdinJiFopwsgAxPrhJ"),
            AquaFarms::whETHUSDC => static_pubkey!("FAgiNoPVnBDXaAytHwoZ36xRMt7h1QZiAmT6ikD5dA98"),
            AquaFarms::whETHSOL => static_pubkey!("H4PuQ5Ly11bYYHVyZ6dSFKARheKaryYyz991ctkFxBXU"),
            AquaFarms::SAMOUSDC => static_pubkey!("TuEvcp71fWQbRXZjW1JEKkYrNpAS4EpgibMvNeYar9j"),
            AquaFarms::SHDWSOL => static_pubkey!("GL62E2J5syXiNpude4vLiBeivvSiVTNwPRyYXWnV32d1"),
            AquaFarms::SHDWUSDC => static_pubkey!("HgM3TutTswwM3LV8LiRj1uVjK8NfeTgq9aagfLLjrY3g"),
            AquaFarms::BASISUSDC => static_pubkey!("GtRcfta8aD8BbZqAZV2gaWELSZBe4qKvexTWqSRUvdYw"),
            AquaFarms::stSOLUSDC => static_pubkey!("8iyvVva8sg79HdLSokCLFF3M1sAmagByTSz4XeQHQ6Vk"),
            AquaFarms::GSTUSDC => static_pubkey!("9swuhhrnLGDzXKAicByxMYBnSANJojeaaCJ3uENWuijq"),
            AquaFarms::stSOLwUST => static_pubkey!("EQBy5YqWkLxr1xx1CZh2dDdX57XR9Ata4jNJebtuR61h"),
            AquaFarms::sRLYSOL => static_pubkey!("8phz73Nyk4XKk7btjEJbEVyvFkcXQUvVEsYcjvoxwrBf"),
            AquaFarms::GMTUSDC => static_pubkey!("FPi9U6fLhVBggWdTwsPgByjEps6w8DiMyU5LqjLtvXnd"),
        }
    }
    pub fn global_base_token_vault_dd(&self) -> Pubkey {
        match self {
            AquaFarms::ATLASUSDC => static_pubkey!("Bu3epZQvoSmUJtzAJWH8v91HFwbc9bRN6B9hrjGojFUW"),
            AquaFarms::POLISUSDC => static_pubkey!("7mymkVgdjcqbDvyQENVnc5hvWLBKFb4G84bnu2eR9aug"),
            AquaFarms::SAMOUSDC => static_pubkey!("Be246fsrPT11xn9aqKt4TciTXwgwmMBKyosmgTr76Tbp"),
            AquaFarms::SHDWSOL => static_pubkey!("JDofVo2hWtwimYbLfP6AXG5NT5zuWWDLEqtxhFZpqmd1"),
            AquaFarms::SHDWUSDC => static_pubkey!("2wEEmMHG668WvfjjbBVtHsbDXSnEjgWWL2uQ53pLs9cA"),
            AquaFarms::BASISUSDC => static_pubkey!("DaHbgd7j9aCmWaidevgPw1YiwnvRt2Q7HYTWVXmeqGmF"),
            AquaFarms::stSOLUSDC => static_pubkey!("8hy1DCbSuqmDJMCxVi4VR47kk8wiVG9SJos1iFe6UBdy"),
            AquaFarms::stSOLwUST => static_pubkey!("9AznnrVmheoiJ2zDwgtmEaxzvj9kU4QJMvt8szZwqitV"),
            AquaFarms::sRLYSOL => static_pubkey!("6k1FjHoaXnZ5tcHmb8LFa7HZXsFXtjFizu62tDUSnY4Z"),

            _ => panic!(":ruhroh:"),
        }
    }
    pub fn farm_token_mint(&self) -> Pubkey {
        match self {
            AquaFarms::SOLUSDC => Pubkey::new_from_array([
                211, 195, 73, 253, 242, 30, 13, 219, 43, 109, 139, 112, 230, 229, 249, 222, 211,
                197, 234, 98, 208, 172, 121, 49, 220, 40, 57, 101, 39, 244, 182, 176,
            ]),
            // Pubkey::from_str("FFdjrSvNALfdgxANNpt3x85WpeVMdQSH5SEP2poM8fcK").unwrap(),
            AquaFarms::ORCAUSDC => Pubkey::new_from_array([
                231, 221, 251, 184, 191, 192, 224, 13, 129, 31, 151, 52, 187, 103, 19, 160, 20, 46,
                89, 130, 121, 226, 142, 160, 174, 141, 93, 48, 83, 254, 145, 193,
            ]),
            // Pubkey::from_str("Gc7W5U66iuHQcC1cQyeX9hxkPF2QUVJPTf1NWbW8fNrt").unwrap(),
            AquaFarms::ORCASOL => Pubkey::new_from_array([
                149, 216, 231, 175, 211, 185, 34, 215, 99, 139, 222, 219, 24, 141, 55, 16, 101,
                225, 153, 118, 36, 103, 109, 54, 13, 105, 211, 250, 227, 218, 183, 192,
            ]),
            // Pubkey::from_str("B5waaKnsmtqFawPspUwcuy1cRjAC7u2LrHSwxPSxK4sZ").unwrap(),
            AquaFarms::USDCUSDT => Pubkey::new_from_array([
                233, 215, 112, 99, 5, 226, 195, 181, 140, 83, 88, 43, 200, 64, 232, 192, 179, 60,
                93, 31, 246, 119, 103, 59, 164, 153, 250, 115, 98, 166, 247, 228,
            ]),
            // Pubkey::from_str("GjpXgKwn4VW4J2pZdS3dovM58hiXWLJtopTfqG83zY2f").unwrap(),
            AquaFarms::ETHUSDC => Pubkey::new_from_array([
                240, 230, 219, 212, 122, 182, 168, 193, 78, 75, 254, 103, 124, 145, 222, 147, 73,
                38, 242, 150, 184, 117, 212, 134, 55, 17, 191, 22, 198, 175, 22, 25,
            ]),
            // Pubkey::from_str("HDP2AYFmvLz6sWpoSuNS62JjvW4HjMKp7doXucqpWN56").unwrap(),
            AquaFarms::SOLUSDT => Pubkey::new_from_array([
                89, 98, 172, 192, 26, 67, 35, 125, 100, 123, 59, 23, 211, 52, 109, 118, 79, 179,
                66, 106, 62, 14, 7, 94, 134, 245, 140, 106, 61, 136, 21, 203,
            ]),
            // Pubkey::from_str("71vZ7Jvu8fTyFzpX399dmoSovoz24rVbipLrRn2wBNzW").unwrap(),
            AquaFarms::ETHSOL => static_pubkey!("CGFTRh4jKLPbS9r4hZtbDfaRuC7qcA8rZpbLnVTzJBer"),
            AquaFarms::mSOLSOL => static_pubkey!("3RTGL7gPF4V1ns1AeGFApT7cBEGVDfmJ77DqQi9AC6uG"),
            AquaFarms::ATLASUSDC => static_pubkey!("HFmY1ggCsCky1zJ1sfdkNR4zb3u5n38YNRdf4vsGu17t"),
            AquaFarms::POLISUSDC => static_pubkey!("63JUKLnCAuNMPSPioEgbjjzp9Qk8qSEEM8eZqEtPqfLU"),
            AquaFarms::SOCNUSDC => static_pubkey!("7YFfqZGTxkj3Zeq3Et23kMznCaEYZ1WBZDt6CVrxwfqd"),
            AquaFarms::SOCNSOL => static_pubkey!("CNqmEKGjZUUARVFHcz4w9CvX5pR8Ae2c6imHDNqsbxgj"),
            AquaFarms::whETHUSDC => static_pubkey!("B11Xp26xU2gzjToJEuGswvr6Jtidfh4GRUyCWzWMNdQZ"),
            AquaFarms::whETHSOL => static_pubkey!("FkHQBBZGh5GS4GcXpcVksKYUUkLTNn6Yk1PCMxucR2AK"),
            AquaFarms::SAMOUSDC => static_pubkey!("9voVuTq1S9bFZkF2Jo44HoVG63w2xDRT8eBzB23YbQud"),
            AquaFarms::SHDWSOL => static_pubkey!("BDStVBt4NS5bfda25ubK51kVRioV4yjKKCPbe96jeEms"),
            AquaFarms::SHDWUSDC => static_pubkey!("7WWHfufv8vuBC1x7GXA3pu7kgNhEQkXoq3CtbaQihAJ9"),
            AquaFarms::BASISUSDC => static_pubkey!("4yx2aHMa7N4m1uUaBRy9QPtpstw3HFPtvcCPJQaGFHKL"),
            AquaFarms::stSOLUSDC => static_pubkey!("3u2dNfGuU6C3vmSg5EvLPUpX57b3niqhWBV5Gc3WDEf5"),
            AquaFarms::GSTUSDC => static_pubkey!("72vxFxfeSN2DRKmSQAkJCoFBNYb2WNevyaDh4v2t8TqP"),
            AquaFarms::stSOLwUST => static_pubkey!("3Q44iV4URXdbS4Tk1PGs5VdWQoCxHB7zdcdMnemo8jfH"),
            AquaFarms::sRLYSOL => static_pubkey!("6gkZ7QUmxmwPLS2NK3Dr6YHtTPZs6GQrkA595WSx5iLe"),
            AquaFarms::GMTUSDC => static_pubkey!("8EnEoVX1aXkzbTzhrqDQ2aVGybbPpeWZDCYEGjjw1dyG"),
        }
    }

    pub fn farm_token_mint_dd(&self) -> Pubkey {
        match self {
            AquaFarms::ATLASUSDC => static_pubkey!("894ptAFT7d3inPsWTniCGL2NZpJDiXGvFZFfuHXA1w8F"),
            AquaFarms::POLISUSDC => static_pubkey!("FE1QJzi5RA5aKnTfSV3DAMN3z4uHUzSR5Z4drs9S5vB"),
            AquaFarms::SAMOUSDC => static_pubkey!("EdfAy8jwnvU1z61UaFUjwoRPFgD3UkkPvnhEBZjzwhv8"),
            AquaFarms::SHDWSOL => static_pubkey!("8HgXuNigmLvfsgDun1vQso6pBuj7sVvDqpcergjtu3dz"),
            AquaFarms::SHDWUSDC => static_pubkey!("HPv7XJ16t4pZVNTBPHoYY19xiv4pHjjSbarE7Km3jJ1R"),
            AquaFarms::BASISUSDC => static_pubkey!("8XtNSYBhLHa4cYzNsXd6yDAweMECumrxFJ7F2qxk2xN"),
            AquaFarms::stSOLUSDC => static_pubkey!("CejKA1pePxny3iprRDEyiojfTKNxxX2bjmKToDGZqwvh"),
            AquaFarms::stSOLwUST => static_pubkey!("DxiftFoeRxHk15N4rDYzpwtGhfK3LqSn4gWDCaEkMksE"),
            AquaFarms::sRLYSOL => static_pubkey!("4Xtia3w6AKBNPCYmFZCAQV8CNwUmHYrVQiNMEGdMkRMg"),

            _ => panic!(":ruhroh:"),
        }
    }

    pub fn global_reward_token_vault(&self) -> Pubkey {
        match self {
            AquaFarms::SOLUSDC => Pubkey::new_from_array([
                11, 52, 73, 179, 160, 153, 13, 228, 162, 235, 45, 14, 33, 243, 70, 164, 122, 136,
                174, 175, 1, 39, 205, 82, 151, 192, 72, 77, 255, 201, 16, 35,
            ]),
            // Pubkey::from_str("kjjFC8RAF7GuBQ9iYgyTcPmvsRafJ2Ec2AmoS6DjakJ").unwrap(),
            AquaFarms::ORCAUSDC => Pubkey::new_from_array([
                181, 208, 25, 198, 68, 33, 69, 245, 80, 29, 130, 116, 185, 95, 241, 42, 131, 236,
                135, 156, 113, 210, 143, 11, 12, 229, 154, 167, 67, 122, 154, 60,
            ]),
            // Pubkey::from_str("DEiqe2Ta9TRMRtWdBqiFV13dhVrqCeG8MMmVwywvXvJo").unwrap(),
            AquaFarms::ORCASOL => Pubkey::new_from_array([
                169, 255, 32, 179, 177, 99, 192, 192, 32, 132, 52, 115, 105, 234, 70, 28, 225, 177,
                201, 238, 182, 36, 231, 174, 189, 142, 187, 35, 139, 198, 39, 114,
            ]),
            // Pubkey::from_str("CSbYA7Cd65Vis2oqX797zmnWmpgENmqrPdmPbTbRPykd").unwrap(),
            AquaFarms::USDCUSDT => Pubkey::new_from_array([
                141, 209, 161, 151, 137, 242, 11, 165, 64, 7, 139, 155, 49, 187, 146, 235, 11, 116,
                144, 86, 144, 247, 232, 68, 60, 99, 9, 96, 124, 91, 14, 81,
            ]),
            // Pubkey::from_str("AYbtHmuJxXpo91m988UdyTtzC6J72WvMAW7XkXqFhAbz").unwrap(),
            AquaFarms::ETHUSDC => Pubkey::new_from_array([
                124, 30, 63, 149, 46, 134, 4, 131, 176, 54, 212, 216, 103, 124, 157, 45, 11, 188,
                116, 1, 156, 93, 215, 183, 87, 82, 111, 110, 233, 134, 213, 180,
            ]),
            // Pubkey::from_str("9MWJmWVAGQ9C9SxwWKidStAA8HjDHpnZ7KfKgVJdrNtj").unwrap(),
            AquaFarms::SOLUSDT => Pubkey::new_from_array([
                238, 115, 75, 60, 198, 23, 230, 178, 209, 70, 119, 252, 220, 217, 26, 47, 169, 15,
                253, 114, 251, 105, 226, 127, 81, 235, 67, 113, 112, 70, 174, 93,
            ]),
            // Pubkey::from_str("H3ozvCeEwnsqnM2naCnXVxLLwH2XPC5kU8BH97XDpDwS").unwrap(),
            AquaFarms::ETHSOL => static_pubkey!("FYTTVMqWPzbnhTsukgiWmPiNJam4yLTxHM9mpzdan2zo"),
            AquaFarms::mSOLSOL => static_pubkey!("7dpUACKvEiuq5kyoGtgiA131hYwdxfFhEeD5TMT4mnzG"),
            AquaFarms::ATLASUSDC => static_pubkey!("7AxW2s5dqKphs9pjVzwNsVHqSznEyLkrtu3w61TQNvHd"),
            AquaFarms::POLISUSDC => static_pubkey!("BsgUyVxojJCtA11JxJ8R7HorsdKogGf8U66rUouZkc7X"),
            AquaFarms::SOCNUSDC => static_pubkey!("5W59sjf3jXgA6JEuUgrTZPdU3zEJ8WvXB21uNMY3aWGb"),
            AquaFarms::SOCNSOL => static_pubkey!("AUKieh8bcCMYttasy34ft5KoZ7SbQepk83KE3eeoKJv5"),
            AquaFarms::whETHUSDC => static_pubkey!("EAVw4je8KWozucNKMrJDnN1xX6Fv4CpmYsG23N3c3Kjz"),
            AquaFarms::whETHSOL => static_pubkey!("2kdi5HdZAdF7mfzaZtbog5Jo2VvzZe8V483qTCwGae4Z"),
            AquaFarms::SAMOUSDC => static_pubkey!("F8vsyagqHVJPL6whBs5BCTdfAcESRBGfGZbjKtciYzUM"),
            AquaFarms::SHDWSOL => static_pubkey!("5tdGnUMzSKiBmK9K8MpeNFYZimZNKoVFga8vm4iAMQnt"),
            AquaFarms::SHDWUSDC => static_pubkey!("HP6oppmHZr3erGPEGcotgcx8y4PPvdZYcC8PE8pU2htu"),
            AquaFarms::BASISUSDC => static_pubkey!("2CAFkxSUTSUhVDncagnRXooi6wfeakVhc3WB4bEZF8K7"),
            AquaFarms::stSOLUSDC => static_pubkey!("12QCfo5SdFuyC9P5oioken91gCYK5vnRbXaDVuV5KLpZ"),
            AquaFarms::GSTUSDC => static_pubkey!("9MboiCmp7Gas6nRX39oRcFge66Efkn5Ui9fz7SrQmue9"),
            AquaFarms::stSOLwUST => static_pubkey!("FFjuVr4FYdRGkPwaQREVQWHsBFGwQ1LH6VkmVSJL5Vva"),
            AquaFarms::sRLYSOL => static_pubkey!("9WQhuAMn99aCkZTrREjSzogrceuPrrtfXxmjKymgqNzt"),
            AquaFarms::GMTUSDC => static_pubkey!("9rc6Cyw21Kx8ofamntNVWa9v8vza7U4hGwxne764qL25"),
        }
    }

    pub fn global_reward_token_vault_dd(&self) -> Pubkey {
        match self {
            AquaFarms::ATLASUSDC => static_pubkey!("H6xDcxgbV4W9FhiR2VQECSxavSzJHnRnmPzoDWtTc2Qt"),
            AquaFarms::POLISUSDC => static_pubkey!("G3FdVWQ8CqhLA7bVMRmDUxd3W7WhEqiqUz4fGDGwFD94"),
            AquaFarms::SAMOUSDC => static_pubkey!("2oQLWYCp6eaoF1aZfpwHhAbyD91FwyWssL3nrbTL4EP7"),
            AquaFarms::SHDWSOL => static_pubkey!("cHjA2dvYNtNRHcdPcGRJ2iivzDCkiR873QmLCDog8Cn"),
            AquaFarms::SHDWUSDC => static_pubkey!("42Ry7Q5886ZYgiC97W7iSCTGo3Pak3Y8jG8gZHgipa2D"),
            AquaFarms::BASISUSDC => static_pubkey!("8kmH4C6nek3hLgzVeb3xwXu4oF7zx8emoRnuGjwbwkgp"),
            AquaFarms::stSOLUSDC => static_pubkey!("8uGgbZDqb8tSzoGJRfsTrpoWAkna9oaWpGSuXaxzvEwr"),
            AquaFarms::stSOLwUST => static_pubkey!("9q9GJtWVcDseWg6ACyPdkShxACprXeZN8angkE1DAcNJ"),
            AquaFarms::sRLYSOL => static_pubkey!("5HkPsuYBHwc3TSRzDCUVyqnD53fuaPhAjE29BaA5TiYg"),
            _ => panic!(":ruhroh:"),
        }
    }

    pub fn convert_authority(&self) -> Pubkey {
        match self {
            AquaFarms::SOLUSDC => Pubkey::new_from_array([
                5, 45, 224, 127, 169, 130, 80, 133, 220, 110, 196, 234, 13, 176, 7, 212, 214, 161,
                151, 81, 227, 225, 6, 96, 150, 214, 45, 87, 246, 36, 180, 200,
            ]),
            // Pubkey::from_str("MDcWkwPqr5HrA91g4GGax7bVP1NDDetnR12nGhoAdYj").unwrap(),
            AquaFarms::ORCAUSDC => Pubkey::new_from_array([
                75, 209, 44, 225, 217, 73, 114, 100, 70, 98, 191, 129, 218, 153, 39, 72, 94, 191,
                54, 209, 157, 161, 247, 135, 17, 98, 199, 174, 1, 123, 184, 154,
            ]),
            // Pubkey::from_str("66xaEjFoYfRcspc18oDj61mXDyznr9zam6tFNeqvs2jK").unwrap(),
            AquaFarms::ORCASOL => Pubkey::new_from_array([
                120, 195, 224, 252, 123, 52, 189, 161, 232, 76, 2, 126, 122, 180, 139, 33, 179,
                160, 236, 141, 242, 6, 154, 181, 10, 130, 214, 104, 89, 60, 169, 149,
            ]),
            // Pubkey::from_str("98RAHBKRTTC87nNwug1GEAnLVgouk9nRaa3u14jrp6Zz").unwrap(),
            AquaFarms::USDCUSDT => Pubkey::new_from_array([
                67, 113, 194, 124, 124, 231, 249, 53, 129, 165, 215, 108, 52, 35, 47, 198, 140,
                248, 155, 228, 218, 144, 103, 253, 81, 239, 121, 121, 204, 133, 122, 169,
            ]),
            // Pubkey::from_str("5YGvg6mfuvJtHdVWDXTs4sYy6GwQAUduK8qurDcL111S").unwrap(),
            AquaFarms::ETHUSDC => Pubkey::new_from_array([
                182, 0, 38, 46, 41, 108, 211, 238, 40, 149, 185, 239, 133, 197, 30, 72, 230, 106,
                204, 184, 71, 83, 239, 59, 120, 68, 96, 218, 133, 104, 102, 118,
            ]),
            // Pubkey::from_str("DFTLJrgsn7cLNX9hbqiUwM8C1y6f7AfyvEmbsFSkjQNR").unwrap(),
            AquaFarms::SOLUSDT => Pubkey::new_from_array([
                201, 216, 113, 84, 137, 88, 184, 69, 89, 37, 54, 247, 184, 10, 244, 105, 203, 174,
                36, 171, 218, 57, 170, 226, 35, 44, 201, 148, 123, 167, 124, 41,
            ]),
            // Pubkey::from_str("EavNUagNtD7DEdV4atcm3dEBXafARKCNJyNkyfz426m6").unwrap(),
            AquaFarms::ETHSOL => static_pubkey!("HXY2Vvj2XyqiPNXV3PhM9YYKgfjqzXUX4tUFRnvqihdY"),
            AquaFarms::mSOLSOL => static_pubkey!("CtXKDXJ4wzgto48QQFANestEgtov5dJRrs9qpRw7BV1h"),
            AquaFarms::ATLASUSDC => static_pubkey!("8tn2JVYGM1JwCEY3QHJxjRjKsmVbvdcqM277GC3yTot6"),
            AquaFarms::POLISUSDC => static_pubkey!("3s5MDuUTfXEAKTbBvcCqAnShFMfdr8s8DoNwrfNBJyrM"),
            AquaFarms::SOCNUSDC => static_pubkey!("DwCwY8Qnh7FniMEYv1cnK841F1Sbc2q4piGZv6R8EzPt"),
            AquaFarms::SOCNSOL => static_pubkey!("9Ccir8HvwwLihY1tS6sFEWYkRSCcL4yXVmCVDFS4dsYr"),
            AquaFarms::whETHUSDC => static_pubkey!("GKjz16uLrWTMe9upsR1PhXs4CL4R6NCAAMEhTPbWaLcr"),
            AquaFarms::whETHSOL => static_pubkey!("DnsJave9pHa4Q4p7pdyaCiHiYAD7vTUR1do5XQVSRCzr"),
            AquaFarms::SAMOUSDC => static_pubkey!("92oB7qcj7zCPmxmUPWfN3a8PUpyoKi7NGfR6JjjLMphp"),
            AquaFarms::SHDWUSDC => static_pubkey!("GjhS6HRdhCGZyVHfKNP1kdeQcPnw3b49Yyk615U9Jmwg"),
            AquaFarms::SHDWSOL => static_pubkey!("3Neyx6gnUEVKkKCZmupU9S5NVh2F2CJHooC8XzX33n36"),
            AquaFarms::BASISUSDC => static_pubkey!("37zeuhxH2ANPw1AQb8jQ8EkZDYoCftDV5qSXM5TGUd6M"),
            AquaFarms::stSOLUSDC => static_pubkey!("4G1mhdPXaUUf2yvCHnZSejDCpPxS7ymPzjYHNfzJqdE9"),
            AquaFarms::GSTUSDC => static_pubkey!("DFRXY2zC2JAHK2jMzoKMwydYhfgoaCXibNrhcb87sqYD"),
            AquaFarms::stSOLwUST => static_pubkey!("8za7LwVuYbP2PivS3KbeohpUkSmRqLdTDxhrVzsHyFZf"),
            AquaFarms::sRLYSOL => static_pubkey!("9aHRFSsmRUuy1SZcjxBswAV4pcXrror35e8JK4WL2ny8"),
            AquaFarms::GMTUSDC => static_pubkey!("48T3au6xpUCz2Eeg6xgc3XJmCUTTH4qwmq1BytihxQMJ"),
        }
    }

    pub fn convert_authority_dd(&self) -> Pubkey {
        match self {
            AquaFarms::ATLASUSDC => static_pubkey!("6HDxNtWVFrgKdce3iaRGwRf9tLhFdPHfxJyzEo8TPrEo"),
            AquaFarms::POLISUSDC => static_pubkey!("F56GFg8TdADqmNfqrER21v6gP85eRtVVNjEPVMTd616r"),
            AquaFarms::SAMOUSDC => static_pubkey!("Cm3wcJtZEPYm9rm39LQNND7pxuL27539yLoMP7LcrLF7"),
            AquaFarms::SHDWUSDC => static_pubkey!("3KdYfjBcVR5t7Qq3i4mc3dGxvcZ95EhdX3wPWG7zMVUd"),
            AquaFarms::SHDWSOL => static_pubkey!("5ZjatkykvKphGB6Ng9Tw4Bud1fhpJ8KQHq4PS9YRiK2J"),
            AquaFarms::BASISUSDC => static_pubkey!("EWXsQ5XMMn1tesDhtuMhgH5e5wg7hWgsQPZxGhaZdBaL"),
            AquaFarms::stSOLUSDC => static_pubkey!("2hLYdBzRaHWHE4h6FNHbUi983aGknESf2hYG3QEiybPa"),
            AquaFarms::stSOLwUST => static_pubkey!("EQn9JfRjpHunU3nGcnizPUphCi8UoT844ZFcGsQ7ZVb2"),
            AquaFarms::sRLYSOL => static_pubkey!("CoWFdWwgAy5nfZsXv4hDskzekGuwyz95ddpdiuYoZm1D"),

            _ => panic!(":ruhroh:"),
        }
    }

    // this is the swap account
    pub fn account(&self) -> Pubkey {
        match self {
            AquaFarms::SOLUSDC => Pubkey::new_from_array([
                197, 35, 245, 131, 249, 110, 212, 253, 189, 138, 222, 22, 95, 141, 254, 85, 51,
                193, 15, 160, 33, 162, 42, 63, 49, 229, 74, 220, 2, 3, 22, 97,
            ]),
            //Pubkey::from_str("EGZ7tiLeH62TPV1gL8WwbXGzEPa9zmcpVnnkPKKnrE2U").unwrap(),
            AquaFarms::ORCAUSDC => Pubkey::new_from_array([
                26, 237, 168, 53, 128, 224, 107, 139, 226, 199, 179, 245, 231, 89, 10, 105, 200,
                190, 188, 55, 198, 208, 3, 218, 37, 41, 159, 156, 162, 101, 165, 159,
            ]),
            //Pubkey::from_str("2p7nYbtPBgtmY69NsE8DAW6szpRJn7tQvDnqvoEWQvjY").unwrap(),
            AquaFarms::USDCUSDT => Pubkey::new_from_array([
                208, 6, 247, 254, 42, 137, 109, 188, 117, 103, 175, 45, 246, 116, 129, 245, 244,
                136, 96, 106, 72, 174, 146, 169, 220, 206, 184, 111, 239, 165, 12, 58,
            ]), // Pubkey::from_str("F13xvvx45jVGd84ynK3c8T89UejQVxjCLtmHfPmAXAHP").unwrap(),
            AquaFarms::ORCASOL => Pubkey::new_from_array([
                23, 65, 183, 210, 120, 223, 34, 172, 145, 136, 23, 26, 89, 95, 45, 192, 98, 15, 44,
                143, 110, 130, 85, 115, 26, 99, 250, 101, 98, 44, 37, 83,
            ]),
            // Pubkey::from_str("2ZnVuidTHpi5WWKUwFXauYGhvdT9jRKYv5MDahtbwtYr").unwrap(),
            AquaFarms::ETHUSDC => {
                Pubkey::from_str("FgZut2qVQEyPBibaTJbbX2PxaMZvT1vjDebiVaDp5BWP").unwrap()
            }
            AquaFarms::SOLUSDT => {
                Pubkey::from_str("Dqk7mHQBx2ZWExmyrR2S8X6UG75CrbbpK2FSBZsNYsw6").unwrap()
            }
            AquaFarms::ETHSOL => static_pubkey!("EuK3xDa4rWuHeMQCBsHf1ETZNiEQb5C476oE9u9kp8Ji"),
            AquaFarms::mSOLSOL => static_pubkey!("9EQMEzJdE2LDAY1hw1RytpufdwAXzatYfQ3M2UuT9b88"),
            AquaFarms::ATLASUSDC => static_pubkey!("3V5sjXj1mrWjjB1Xt6Xwp554QwHE5fppGSxbk4GzAtEW"),
            AquaFarms::POLISUSDC => static_pubkey!("CdKPtCb5fBRaGFS4bJgytfReeHuFyhpe9YUyWHPnEWZG"),
            AquaFarms::SOCNUSDC => static_pubkey!("6Gh36sNXrGWYiWr999d9iZtqgnipJbWuBohyHBN1cJpS"),
            AquaFarms::SOCNSOL => static_pubkey!("2q6UMko5kTnv866W9MTeAFau94pLpsdeNjDdSYSgZUXr"),
            AquaFarms::whETHUSDC => static_pubkey!("4reGGLbesqpAeAZdAJv9hhgA2tgj45oGcyRuEvhATdMm"),
            AquaFarms::whETHSOL => static_pubkey!("FcEro2uFpHcb7Z785CBs6q12KMJqUJKa8VTXPi4TTBMf"),
            AquaFarms::SAMOUSDC => static_pubkey!("Epvp7qMYAF21VVjacdB3VfKn6nnXQSF4rGYu8sD6Bkow"),
            AquaFarms::SHDWUSDC => static_pubkey!("25bQ6UzZpgFgnU7MqZdqM9Axi6oJunytRL2LgXruDWZB"),
            AquaFarms::SHDWSOL => static_pubkey!("E3fxkJGNNAWf5xXDfMdq5qofBVkQtLKxkP7gG6Up21Ts"),
            AquaFarms::BASISUSDC => static_pubkey!("EWXsQ5XMMn1tesDhtuMhgH5e5wg7hWgsQPZxGhaZdBaL"),
            AquaFarms::stSOLUSDC => static_pubkey!("EfK84vYEKT1PoTJr6fBVKFbyA7ZoftfPo2LQPAJG1exL"),
            AquaFarms::GSTUSDC => static_pubkey!("87E4KtN7F4LivKhjqXaoQAvS3a8HnM4DnMUrbMrkVvXY"),
            AquaFarms::stSOLwUST => static_pubkey!("9F3J6RY7PTkDb3SUUpg725uXyCceBGCpZrtmYGJwgMwF"),
            AquaFarms::sRLYSOL => static_pubkey!("Df7DkQRXEpPM5basbYHi45268hmR8m7YrtraPdGgj6R6"),
            AquaFarms::GMTUSDC => static_pubkey!("46GcZFgznxUf6TpoCqJqzMpgMbbJPCAwNn8GThSt9qjC"),
        }
    }
    pub fn pool_fee_account(&self) -> Pubkey {
        match self {
            AquaFarms::SOLUSDC => {
                Pubkey::from_str("8JnSiuvQq3BVuCU3n4DrSTw9chBSPvEMswrhtifVkr1o").unwrap()
            }
            AquaFarms::ORCAUSDC => {
                Pubkey::from_str("7CXZED4jfRp3qdHB9Py3up6v1C4UhHofFvfT6RXbJLRN").unwrap()
            }
            AquaFarms::ORCASOL => {
                Pubkey::from_str("4Zc4kQZhRQeGztihvcGSWezJE1k44kKEgPCAkdeBfras").unwrap()
            }
            AquaFarms::USDCUSDT => {
                Pubkey::from_str("B4RNxMJGRzKFQyTq2Uwkmpyjtew13n7KtdqZy6qgENTu").unwrap()
            }
            AquaFarms::ETHUSDC => {
                Pubkey::from_str("DLWewB12jzGn4wXJmFCddWDeof1Ma4cZYNRv9CP5hTvX").unwrap()
            }
            AquaFarms::SOLUSDT => {
                Pubkey::from_str("BBKgw75FivTYXj85D2AWyVdaTdTWuSuHVXRm1Xu7fipb").unwrap()
            }
            AquaFarms::ETHSOL => static_pubkey!("unxKgWEc71ZiHwMqZs3VLqjcjmZhfTZEg94ZLGvjdMP"),
            AquaFarms::mSOLSOL => static_pubkey!("6j2tt2UVYMQwqG3hRtyydW3odzBFwy3pN33tyB3xCKQ6"),
            AquaFarms::ATLASUSDC => static_pubkey!("CFN4DQ2p3qroX92pPNy3mov3Dw1aCNGLrU5AXHpHxbko"),
            AquaFarms::POLISUSDC => static_pubkey!("3gZQ2YnrXbnRwJj5poffLirF7CwacatvtfGCNRFrbJdr"),
            AquaFarms::SOCNUSDC => static_pubkey!("HsC1Jo38jK3EpoNAkxfoUJhQVPa28anewZpLfeouUNk7"),
            AquaFarms::SOCNSOL => static_pubkey!("42Xzazs9EvjtidvEDrj3JXbDtf6fpTq5XHh96mPctvBV"),
            AquaFarms::whETHSOL => static_pubkey!("YCVJDKdHNi1mhJtWz7QGbBRreMmw1soeipz7wZbQKEK"), // this is likely incorrect as its the one for whETHUSDC
            AquaFarms::whETHUSDC => static_pubkey!("AVw52spXtzFh4bb5ghhpJaDbLx3XWuY85eQNDEo3X1yN"),
            AquaFarms::SAMOUSDC => static_pubkey!("9U8UF7d8kBvsS25XoZnjmVQ9vGkP4BUnHJgfc615BvG1"),
            AquaFarms::SHDWUSDC => static_pubkey!("9wmHbXURZ4zTPSj1KqoRSCdBRGUF7jrURzf7BB39cxM4"),
            AquaFarms::SHDWSOL => static_pubkey!("G9HR4sFJufdUovMGn4qc97r7fhgJCkTDnn4BT2wPWYar"),
            AquaFarms::BASISUSDC => static_pubkey!("4FjEd37W9FExXq85nLeuNWuhUaTwkFdnqewt3E3qoYAh"),
            AquaFarms::stSOLUSDC => static_pubkey!("CJhL3UGesECFt6fvLB3csrGMuHf3M3G78pUzTopUiV8T"),
            AquaFarms::GSTUSDC => static_pubkey!("BynpQprCNjcY2KHeffDKzquyKWvJxikty3donrMT4ZPU"),
            AquaFarms::stSOLwUST => static_pubkey!("5rCbmppxMBHwBjCkLUP6fireQ12cL8LRa26QRUimoxN6"),
            AquaFarms::sRLYSOL => static_pubkey!("B3Ao2fEX2isX8UQ99EuPz3BDzUfQTPeYS7KVvbCnkrXm"),
            AquaFarms::GMTUSDC => static_pubkey!("3pBqsnahNsm6p14FFjtMCGfD1VCQNcUEdNEeSwTGfE2q"),
        }
    }
    pub fn solfarm_vault(&self) -> Pubkey {
        match self {
            AquaFarms::ORCAUSDC => {
                Pubkey::from_str("2Dts63SfTz2yivx57izMcVfDMAdpxuBgqM99ChWeJXun").unwrap()
            }
            AquaFarms::ORCASOL => {
                Pubkey::from_str("CsmpHw2kxqYJGvrk1DXASAEosM1nTWgDiytqa64YSSon").unwrap()
            }
            AquaFarms::SOLUSDC => {
                Pubkey::from_str("5Qk3dT58AmbGvgADpauiUaUccb4EPSUxvJaVHcUzVUT3").unwrap()
            }
            AquaFarms::USDCUSDT => {
                Pubkey::from_str("6HFyXPFZrxUjDkJc9ty2susATjCtbA1SC73WYN5nRsXC").unwrap()
            }
            AquaFarms::ETHUSDC => {
                Pubkey::from_str("7uqa33AAUmsXKduwrv2ck8o6t9WPFGHmKn7DMSWUg6GM").unwrap()
            }
            AquaFarms::SOLUSDT => {
                Pubkey::from_str("2Y9RbhbBKBvErq8DDQeD9tQuUaHS8vMAp7cKgqoDLy4a").unwrap()
            }
            AquaFarms::ETHSOL => static_pubkey!("8xxkkN1Vez555i98wMUBKRWTgXUgsdXMMZppw6cPD4q6"),
            AquaFarms::ATLASUSDC => static_pubkey!("HDKmRtHdod3Yag3Bc2R4BwzZ6kaVFpG9q8bUHeKcfsqH"),
            AquaFarms::POLISUSDC => static_pubkey!("8SpU3XBQPgxivEsqq3m8LdXyPRXonhtKZkfr6uGviLAa"),
            AquaFarms::whETHUSDC => static_pubkey!("Gp8L3oFuwQccXnKrtuScL9btgk7WGFdVZty1bkj8YrUZ"),
            AquaFarms::whETHSOL => static_pubkey!("6GLMd139W3oZH22M1maqgda44QLXjjcsethsKMQqntWV"),
            AquaFarms::SAMOUSDC => static_pubkey!("ECDzEfUgtDKq9dAvdeiiSiW2m4H4AaG5unSJ25dmFgij"),
            AquaFarms::SHDWUSDC => static_pubkey!("EGyBubbg46tJQEE7si2Dj12zizDvy6RoJwZRVS3K8e7W"),
            AquaFarms::SHDWSOL => static_pubkey!("DyqYFtTQYJTbv6zH1exzviECk6eBRDZh7MNHQkTq1Tau"),
            AquaFarms::BASISUSDC => static_pubkey!("E5dMKcCDK3K2FKMv9WWXByk8bzeVossBK5DWA9fAM1xN"),
            AquaFarms::stSOLUSDC => static_pubkey!("G4WWuMytLFz7ZJoVbUWfz6cTFCwHiHzAaL5sMKnhrQaz"),
            AquaFarms::stSOLwUST => static_pubkey!("CdJkFY3LWJiMgJxPp92QRP4nrWQpkw4hdH9TNE5FbPZK"),
            AquaFarms::sRLYSOL => static_pubkey!("5WND625Kzemq1kix5h1FybZvP4yXdwxHDzkEpbK6ZWoE"),
            AquaFarms::GMTUSDC => static_pubkey!("94oj1i2ujRTDMFeBK4trvXfjvM3D9FAMvpQbri9aaHmQ"),
            _ => panic!(":ruhroh:"),
        }
    }
    pub fn is_double_dip(&self) -> bool {
        match self {
            AquaFarms::ATLASUSDC
            | AquaFarms::POLISUSDC
            | AquaFarms::SAMOUSDC
            | AquaFarms::SHDWUSDC
            | AquaFarms::SHDWSOL
            | AquaFarms::BASISUSDC
            | AquaFarms::stSOLUSDC
            | AquaFarms::stSOLwUST
            | AquaFarms::sRLYSOL => true,
            _ => false,
        }
    }
}

impl From<super::Farms> for AquaFarms {
    fn from(val: crate::accounts::Farms) -> AquaFarms {
        match val {
            crate::accounts::Farms::OrcaSolVault => AquaFarms::ORCASOL,
            crate::accounts::Farms::OrcaUsdcVault => AquaFarms::ORCAUSDC,
            crate::accounts::Farms::SolUsdcOrcaVault => AquaFarms::SOLUSDC,
            crate::accounts::Farms::SolUsdtOrcaVault => AquaFarms::SOLUSDT,
            crate::accounts::Farms::EthUsdcOrcaVault => AquaFarms::ETHUSDC,
            crate::accounts::Farms::EthSolOrcaVault => AquaFarms::ETHSOL,
            crate::accounts::Farms::AtlasUsdcOrcaVault => AquaFarms::ATLASUSDC,
            crate::accounts::Farms::PolisUsdcOrcaVault => AquaFarms::POLISUSDC,
            crate::accounts::Farms::whEthSolOrcaVault => AquaFarms::whETHSOL,
            crate::accounts::Farms::whEthUsdcOrcaVault => AquaFarms::whETHUSDC,
            crate::accounts::Farms::SamoUsdcOrcaVault => AquaFarms::SAMOUSDC,
            crate::accounts::Farms::ShdwUsdcOrcaVault => AquaFarms::SHDWUSDC,
            crate::accounts::Farms::ShdwSolOrcaVault => AquaFarms::SHDWSOL,
            crate::accounts::Farms::BasisUsdcOrcVault => AquaFarms::BASISUSDC,
            crate::accounts::Farms::stSolUsdcOrcaVault => AquaFarms::stSOLUSDC,
            crate::accounts::Farms::GstUsdcOrcaVault => AquaFarms::GSTUSDC,
            crate::accounts::Farms::stSolwUstOrcaVault => AquaFarms::stSOLwUST,
            crate::accounts::Farms::sRlySolOrcaVault => AquaFarms::sRLYSOL,
            crate::accounts::Farms::GmtUsdcOrcaVault => AquaFarms::GMTUSDC,
            _ => panic!("farm is not an orca farm"),
        }
    }
}
