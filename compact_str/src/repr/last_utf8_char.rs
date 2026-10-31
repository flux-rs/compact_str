use alloc::string::String;

/// [`LastByte`] is an unsigned 8-bit integer data type that has a valid range of `[0, 217]`.
/// Excluding `[218, 255]` allows the Rust compiler to use these values as niches.
///
/// Specifically the compiler can use a value in this range to encode the `None` variant of
/// `Option<LastByte>` allowing:
/// `std::mem::size_of::<LastByte> == std::mem::size_of::<Option<LastByte>>()`
#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
#[flux_rs::refined_by(val: int)]
#[flux_rs::invariant(0 <= val && val <= 217)]
pub(crate) enum LastByte {
    // single character, ASCII:
    #[flux_rs::variant(LastByte[0])]
    V0 = 0,
    #[flux_rs::variant(LastByte[1])]
    V1 = 1,
    #[flux_rs::variant(LastByte[2])]
    V2 = 2,
    #[flux_rs::variant(LastByte[3])]
    V3 = 3,
    #[flux_rs::variant(LastByte[4])]
    V4 = 4,
    #[flux_rs::variant(LastByte[5])]
    V5 = 5,
    #[flux_rs::variant(LastByte[6])]
    V6 = 6,
    #[flux_rs::variant(LastByte[7])]
    V7 = 7,
    #[flux_rs::variant(LastByte[8])]
    V8 = 8,
    #[flux_rs::variant(LastByte[9])]
    V9 = 9,
    #[flux_rs::variant(LastByte[10])]
    V10 = 10,
    #[flux_rs::variant(LastByte[11])]
    V11 = 11,
    #[flux_rs::variant(LastByte[12])]
    V12 = 12,
    #[flux_rs::variant(LastByte[13])]
    V13 = 13,
    #[flux_rs::variant(LastByte[14])]
    V14 = 14,
    #[flux_rs::variant(LastByte[15])]
    V15 = 15,
    #[flux_rs::variant(LastByte[16])]
    V16 = 16,
    #[flux_rs::variant(LastByte[17])]
    V17 = 17,
    #[flux_rs::variant(LastByte[18])]
    V18 = 18,
    #[flux_rs::variant(LastByte[19])]
    V19 = 19,
    #[flux_rs::variant(LastByte[20])]
    V20 = 20,
    #[flux_rs::variant(LastByte[21])]
    V21 = 21,
    #[flux_rs::variant(LastByte[22])]
    V22 = 22,
    #[flux_rs::variant(LastByte[23])]
    V23 = 23,
    #[flux_rs::variant(LastByte[24])]
    V24 = 24,
    #[flux_rs::variant(LastByte[25])]
    V25 = 25,
    #[flux_rs::variant(LastByte[26])]
    V26 = 26,
    #[flux_rs::variant(LastByte[27])]
    V27 = 27,
    #[flux_rs::variant(LastByte[28])]
    V28 = 28,
    #[flux_rs::variant(LastByte[29])]
    V29 = 29,
    #[flux_rs::variant(LastByte[30])]
    V30 = 30,
    #[flux_rs::variant(LastByte[31])]
    V31 = 31,
    #[flux_rs::variant(LastByte[32])]
    V32 = 32,
    #[flux_rs::variant(LastByte[33])]
    V33 = 33,
    #[flux_rs::variant(LastByte[34])]
    V34 = 34,
    #[flux_rs::variant(LastByte[35])]
    V35 = 35,
    #[flux_rs::variant(LastByte[36])]
    V36 = 36,
    #[flux_rs::variant(LastByte[37])]
    V37 = 37,
    #[flux_rs::variant(LastByte[38])]
    V38 = 38,
    #[flux_rs::variant(LastByte[39])]
    V39 = 39,
    #[flux_rs::variant(LastByte[40])]
    V40 = 40,
    #[flux_rs::variant(LastByte[41])]
    V41 = 41,
    #[flux_rs::variant(LastByte[42])]
    V42 = 42,
    #[flux_rs::variant(LastByte[43])]
    V43 = 43,
    #[flux_rs::variant(LastByte[44])]
    V44 = 44,
    #[flux_rs::variant(LastByte[45])]
    V45 = 45,
    #[flux_rs::variant(LastByte[46])]
    V46 = 46,
    #[flux_rs::variant(LastByte[47])]
    V47 = 47,
    #[flux_rs::variant(LastByte[48])]
    V48 = 48,
    #[flux_rs::variant(LastByte[49])]
    V49 = 49,
    #[flux_rs::variant(LastByte[50])]
    V50 = 50,
    #[flux_rs::variant(LastByte[51])]
    V51 = 51,
    #[flux_rs::variant(LastByte[52])]
    V52 = 52,
    #[flux_rs::variant(LastByte[53])]
    V53 = 53,
    #[flux_rs::variant(LastByte[54])]
    V54 = 54,
    #[flux_rs::variant(LastByte[55])]
    V55 = 55,
    #[flux_rs::variant(LastByte[56])]
    V56 = 56,
    #[flux_rs::variant(LastByte[57])]
    V57 = 57,
    #[flux_rs::variant(LastByte[58])]
    V58 = 58,
    #[flux_rs::variant(LastByte[59])]
    V59 = 59,
    #[flux_rs::variant(LastByte[60])]
    V60 = 60,
    #[flux_rs::variant(LastByte[61])]
    V61 = 61,
    #[flux_rs::variant(LastByte[62])]
    V62 = 62,
    #[flux_rs::variant(LastByte[63])]
    V63 = 63,
    #[flux_rs::variant(LastByte[64])]
    V64 = 64,
    #[flux_rs::variant(LastByte[65])]
    V65 = 65,
    #[flux_rs::variant(LastByte[66])]
    V66 = 66,
    #[flux_rs::variant(LastByte[67])]
    V67 = 67,
    #[flux_rs::variant(LastByte[68])]
    V68 = 68,
    #[flux_rs::variant(LastByte[69])]
    V69 = 69,
    #[flux_rs::variant(LastByte[70])]
    V70 = 70,
    #[flux_rs::variant(LastByte[71])]
    V71 = 71,
    #[flux_rs::variant(LastByte[72])]
    V72 = 72,
    #[flux_rs::variant(LastByte[73])]
    V73 = 73,
    #[flux_rs::variant(LastByte[74])]
    V74 = 74,
    #[flux_rs::variant(LastByte[75])]
    V75 = 75,
    #[flux_rs::variant(LastByte[76])]
    V76 = 76,
    #[flux_rs::variant(LastByte[77])]
    V77 = 77,
    #[flux_rs::variant(LastByte[78])]
    V78 = 78,
    #[flux_rs::variant(LastByte[79])]
    V79 = 79,
    #[flux_rs::variant(LastByte[80])]
    V80 = 80,
    #[flux_rs::variant(LastByte[81])]
    V81 = 81,
    #[flux_rs::variant(LastByte[82])]
    V82 = 82,
    #[flux_rs::variant(LastByte[83])]
    V83 = 83,
    #[flux_rs::variant(LastByte[84])]
    V84 = 84,
    #[flux_rs::variant(LastByte[85])]
    V85 = 85,
    #[flux_rs::variant(LastByte[86])]
    V86 = 86,
    #[flux_rs::variant(LastByte[87])]
    V87 = 87,
    #[flux_rs::variant(LastByte[88])]
    V88 = 88,
    #[flux_rs::variant(LastByte[89])]
    V89 = 89,
    #[flux_rs::variant(LastByte[90])]
    V90 = 90,
    #[flux_rs::variant(LastByte[91])]
    V91 = 91,
    #[flux_rs::variant(LastByte[92])]
    V92 = 92,
    #[flux_rs::variant(LastByte[93])]
    V93 = 93,
    #[flux_rs::variant(LastByte[94])]
    V94 = 94,
    #[flux_rs::variant(LastByte[95])]
    V95 = 95,
    #[flux_rs::variant(LastByte[96])]
    V96 = 96,
    #[flux_rs::variant(LastByte[97])]
    V97 = 97,
    #[flux_rs::variant(LastByte[98])]
    V98 = 98,
    #[flux_rs::variant(LastByte[99])]
    V99 = 99,
    #[flux_rs::variant(LastByte[100])]
    V100 = 100,
    #[flux_rs::variant(LastByte[101])]
    V101 = 101,
    #[flux_rs::variant(LastByte[102])]
    V102 = 102,
    #[flux_rs::variant(LastByte[103])]
    V103 = 103,
    #[flux_rs::variant(LastByte[104])]
    V104 = 104,
    #[flux_rs::variant(LastByte[105])]
    V105 = 105,
    #[flux_rs::variant(LastByte[106])]
    V106 = 106,
    #[flux_rs::variant(LastByte[107])]
    V107 = 107,
    #[flux_rs::variant(LastByte[108])]
    V108 = 108,
    #[flux_rs::variant(LastByte[109])]
    V109 = 109,
    #[flux_rs::variant(LastByte[110])]
    V110 = 110,
    #[flux_rs::variant(LastByte[111])]
    V111 = 111,
    #[flux_rs::variant(LastByte[112])]
    V112 = 112,
    #[flux_rs::variant(LastByte[113])]
    V113 = 113,
    #[flux_rs::variant(LastByte[114])]
    V114 = 114,
    #[flux_rs::variant(LastByte[115])]
    V115 = 115,
    #[flux_rs::variant(LastByte[116])]
    V116 = 116,
    #[flux_rs::variant(LastByte[117])]
    V117 = 117,
    #[flux_rs::variant(LastByte[118])]
    V118 = 118,
    #[flux_rs::variant(LastByte[119])]
    V119 = 119,
    #[flux_rs::variant(LastByte[120])]
    V120 = 120,
    #[flux_rs::variant(LastByte[121])]
    V121 = 121,
    #[flux_rs::variant(LastByte[122])]
    V122 = 122,
    #[flux_rs::variant(LastByte[123])]
    V123 = 123,
    #[flux_rs::variant(LastByte[124])]
    V124 = 124,
    #[flux_rs::variant(LastByte[125])]
    V125 = 125,
    #[flux_rs::variant(LastByte[126])]
    V126 = 126,
    #[flux_rs::variant(LastByte[127])]
    V127 = 127,

    // trailing byte in a multi-byte UTF-8 sequence
    #[flux_rs::variant(LastByte[128])]
    V128 = 128,
    #[flux_rs::variant(LastByte[129])]
    V129 = 129,
    #[flux_rs::variant(LastByte[130])]
    V130 = 130,
    #[flux_rs::variant(LastByte[131])]
    V131 = 131,
    #[flux_rs::variant(LastByte[132])]
    V132 = 132,
    #[flux_rs::variant(LastByte[133])]
    V133 = 133,
    #[flux_rs::variant(LastByte[134])]
    V134 = 134,
    #[flux_rs::variant(LastByte[135])]
    V135 = 135,
    #[flux_rs::variant(LastByte[136])]
    V136 = 136,
    #[flux_rs::variant(LastByte[137])]
    V137 = 137,
    #[flux_rs::variant(LastByte[138])]
    V138 = 138,
    #[flux_rs::variant(LastByte[139])]
    V139 = 139,
    #[flux_rs::variant(LastByte[140])]
    V140 = 140,
    #[flux_rs::variant(LastByte[141])]
    V141 = 141,
    #[flux_rs::variant(LastByte[142])]
    V142 = 142,
    #[flux_rs::variant(LastByte[143])]
    V143 = 143,
    #[flux_rs::variant(LastByte[144])]
    V144 = 144,
    #[flux_rs::variant(LastByte[145])]
    V145 = 145,
    #[flux_rs::variant(LastByte[146])]
    V146 = 146,
    #[flux_rs::variant(LastByte[147])]
    V147 = 147,
    #[flux_rs::variant(LastByte[148])]
    V148 = 148,
    #[flux_rs::variant(LastByte[149])]
    V149 = 149,
    #[flux_rs::variant(LastByte[150])]
    V150 = 150,
    #[flux_rs::variant(LastByte[151])]
    V151 = 151,
    #[flux_rs::variant(LastByte[152])]
    V152 = 152,
    #[flux_rs::variant(LastByte[153])]
    V153 = 153,
    #[flux_rs::variant(LastByte[154])]
    V154 = 154,
    #[flux_rs::variant(LastByte[155])]
    V155 = 155,
    #[flux_rs::variant(LastByte[156])]
    V156 = 156,
    #[flux_rs::variant(LastByte[157])]
    V157 = 157,
    #[flux_rs::variant(LastByte[158])]
    V158 = 158,
    #[flux_rs::variant(LastByte[159])]
    V159 = 159,
    #[flux_rs::variant(LastByte[160])]
    V160 = 160,
    #[flux_rs::variant(LastByte[161])]
    V161 = 161,
    #[flux_rs::variant(LastByte[162])]
    V162 = 162,
    #[flux_rs::variant(LastByte[163])]
    V163 = 163,
    #[flux_rs::variant(LastByte[164])]
    V164 = 164,
    #[flux_rs::variant(LastByte[165])]
    V165 = 165,
    #[flux_rs::variant(LastByte[166])]
    V166 = 166,
    #[flux_rs::variant(LastByte[167])]
    V167 = 167,
    #[flux_rs::variant(LastByte[168])]
    V168 = 168,
    #[flux_rs::variant(LastByte[169])]
    V169 = 169,
    #[flux_rs::variant(LastByte[170])]
    V170 = 170,
    #[flux_rs::variant(LastByte[171])]
    V171 = 171,
    #[flux_rs::variant(LastByte[172])]
    V172 = 172,
    #[flux_rs::variant(LastByte[173])]
    V173 = 173,
    #[flux_rs::variant(LastByte[174])]
    V174 = 174,
    #[flux_rs::variant(LastByte[175])]
    V175 = 175,
    #[flux_rs::variant(LastByte[176])]
    V176 = 176,
    #[flux_rs::variant(LastByte[177])]
    V177 = 177,
    #[flux_rs::variant(LastByte[178])]
    V178 = 178,
    #[flux_rs::variant(LastByte[179])]
    V179 = 179,
    #[flux_rs::variant(LastByte[180])]
    V180 = 180,
    #[flux_rs::variant(LastByte[181])]
    V181 = 181,
    #[flux_rs::variant(LastByte[182])]
    V182 = 182,
    #[flux_rs::variant(LastByte[183])]
    V183 = 183,
    #[flux_rs::variant(LastByte[184])]
    V184 = 184,
    #[flux_rs::variant(LastByte[185])]
    V185 = 185,
    #[flux_rs::variant(LastByte[186])]
    V186 = 186,
    #[flux_rs::variant(LastByte[187])]
    V187 = 187,
    #[flux_rs::variant(LastByte[188])]
    V188 = 188,
    #[flux_rs::variant(LastByte[189])]
    V189 = 189,
    #[flux_rs::variant(LastByte[190])]
    V190 = 190,
    #[flux_rs::variant(LastByte[191])]
    V191 = 191,

    // Cannot be last character of a UTF-8 sequence (leading byte of a sequence)
    // V192 .. V244,
    // Can never occur in UTF-8 (start for a codepoint > U+10FFFF)
    // V245 .. 255,

    // length marker:
    #[flux_rs::variant(LastByte[192])]
    L0 = 192,
    #[flux_rs::variant(LastByte[193])]
    L1 = 193,
    #[flux_rs::variant(LastByte[194])]
    L2 = 194,
    #[flux_rs::variant(LastByte[195])]
    L3 = 195,
    #[flux_rs::variant(LastByte[196])]
    L4 = 196,
    #[flux_rs::variant(LastByte[197])]
    L5 = 197,
    #[flux_rs::variant(LastByte[198])]
    L6 = 198,
    #[flux_rs::variant(LastByte[199])]
    L7 = 199,
    #[flux_rs::variant(LastByte[200])]
    L8 = 200,
    #[flux_rs::variant(LastByte[201])]
    L9 = 201,
    #[flux_rs::variant(LastByte[202])]
    L10 = 202,
    #[flux_rs::variant(LastByte[203])]
    L11 = 203,
    #[flux_rs::variant(LastByte[204])]
    L12 = 204,
    #[flux_rs::variant(LastByte[205])]
    L13 = 205,
    #[flux_rs::variant(LastByte[206])]
    L14 = 206,
    #[flux_rs::variant(LastByte[207])]
    L15 = 207,
    #[flux_rs::variant(LastByte[208])]
    L16 = 208,
    #[flux_rs::variant(LastByte[209])]
    L17 = 209,
    #[flux_rs::variant(LastByte[210])]
    L18 = 210,
    #[flux_rs::variant(LastByte[211])]
    L19 = 211,
    #[flux_rs::variant(LastByte[212])]
    L20 = 212,
    #[flux_rs::variant(LastByte[213])]
    L21 = 213,
    #[flux_rs::variant(LastByte[214])]
    L22 = 214,
    #[flux_rs::variant(LastByte[215])]
    L23 = 215,

    #[flux_rs::variant(LastByte[216])]
    Heap = 216,
    #[flux_rs::variant(LastByte[217])]
    Static = 217,
}

static_assertions::assert_eq_size!(LastByte, Option<LastByte>, u8);
static_assertions::const_assert!(core::mem::size_of::<String>() <= 24);
