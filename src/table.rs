use crate::util::*;
pub(crate) const fn crc16_table(poly: u16, reflect: bool) -> [u16; 256] {
    let mut table = [0u16; 256];
    table[0] = crc16(poly, reflect, 0);
    table[1] = crc16(poly, reflect, 1);
    table[2] = crc16(poly, reflect, 2);
    table[3] = crc16(poly, reflect, 3);
    table[4] = crc16(poly, reflect, 4);
    table[5] = crc16(poly, reflect, 5);
    table[6] = crc16(poly, reflect, 6);
    table[7] = crc16(poly, reflect, 7);
    table[8] = crc16(poly, reflect, 8);
    table[9] = crc16(poly, reflect, 9);
    table[10] = crc16(poly, reflect, 10);
    table[11] = crc16(poly, reflect, 11);
    table[12] = crc16(poly, reflect, 12);
    table[13] = crc16(poly, reflect, 13);
    table[14] = crc16(poly, reflect, 14);
    table[15] = crc16(poly, reflect, 15);
    table[16] = crc16(poly, reflect, 16);
    table[17] = crc16(poly, reflect, 17);
    table[18] = crc16(poly, reflect, 18);
    table[19] = crc16(poly, reflect, 19);
    table[20] = crc16(poly, reflect, 20);
    table[21] = crc16(poly, reflect, 21);
    table[22] = crc16(poly, reflect, 22);
    table[23] = crc16(poly, reflect, 23);
    table[24] = crc16(poly, reflect, 24);
    table[25] = crc16(poly, reflect, 25);
    table[26] = crc16(poly, reflect, 26);
    table[27] = crc16(poly, reflect, 27);
    table[28] = crc16(poly, reflect, 28);
    table[29] = crc16(poly, reflect, 29);
    table[30] = crc16(poly, reflect, 30);
    table[31] = crc16(poly, reflect, 31);
    table[32] = crc16(poly, reflect, 32);
    table[33] = crc16(poly, reflect, 33);
    table[34] = crc16(poly, reflect, 34);
    table[35] = crc16(poly, reflect, 35);
    table[36] = crc16(poly, reflect, 36);
    table[37] = crc16(poly, reflect, 37);
    table[38] = crc16(poly, reflect, 38);
    table[39] = crc16(poly, reflect, 39);
    table[40] = crc16(poly, reflect, 40);
    table[41] = crc16(poly, reflect, 41);
    table[42] = crc16(poly, reflect, 42);
    table[43] = crc16(poly, reflect, 43);
    table[44] = crc16(poly, reflect, 44);
    table[45] = crc16(poly, reflect, 45);
    table[46] = crc16(poly, reflect, 46);
    table[47] = crc16(poly, reflect, 47);
    table[48] = crc16(poly, reflect, 48);
    table[49] = crc16(poly, reflect, 49);
    table[50] = crc16(poly, reflect, 50);
    table[51] = crc16(poly, reflect, 51);
    table[52] = crc16(poly, reflect, 52);
    table[53] = crc16(poly, reflect, 53);
    table[54] = crc16(poly, reflect, 54);
    table[55] = crc16(poly, reflect, 55);
    table[56] = crc16(poly, reflect, 56);
    table[57] = crc16(poly, reflect, 57);
    table[58] = crc16(poly, reflect, 58);
    table[59] = crc16(poly, reflect, 59);
    table[60] = crc16(poly, reflect, 60);
    table[61] = crc16(poly, reflect, 61);
    table[62] = crc16(poly, reflect, 62);
    table[63] = crc16(poly, reflect, 63);
    table[64] = crc16(poly, reflect, 64);
    table[65] = crc16(poly, reflect, 65);
    table[66] = crc16(poly, reflect, 66);
    table[67] = crc16(poly, reflect, 67);
    table[68] = crc16(poly, reflect, 68);
    table[69] = crc16(poly, reflect, 69);
    table[70] = crc16(poly, reflect, 70);
    table[71] = crc16(poly, reflect, 71);
    table[72] = crc16(poly, reflect, 72);
    table[73] = crc16(poly, reflect, 73);
    table[74] = crc16(poly, reflect, 74);
    table[75] = crc16(poly, reflect, 75);
    table[76] = crc16(poly, reflect, 76);
    table[77] = crc16(poly, reflect, 77);
    table[78] = crc16(poly, reflect, 78);
    table[79] = crc16(poly, reflect, 79);
    table[80] = crc16(poly, reflect, 80);
    table[81] = crc16(poly, reflect, 81);
    table[82] = crc16(poly, reflect, 82);
    table[83] = crc16(poly, reflect, 83);
    table[84] = crc16(poly, reflect, 84);
    table[85] = crc16(poly, reflect, 85);
    table[86] = crc16(poly, reflect, 86);
    table[87] = crc16(poly, reflect, 87);
    table[88] = crc16(poly, reflect, 88);
    table[89] = crc16(poly, reflect, 89);
    table[90] = crc16(poly, reflect, 90);
    table[91] = crc16(poly, reflect, 91);
    table[92] = crc16(poly, reflect, 92);
    table[93] = crc16(poly, reflect, 93);
    table[94] = crc16(poly, reflect, 94);
    table[95] = crc16(poly, reflect, 95);
    table[96] = crc16(poly, reflect, 96);
    table[97] = crc16(poly, reflect, 97);
    table[98] = crc16(poly, reflect, 98);
    table[99] = crc16(poly, reflect, 99);
    table[100] = crc16(poly, reflect, 100);
    table[101] = crc16(poly, reflect, 101);
    table[102] = crc16(poly, reflect, 102);
    table[103] = crc16(poly, reflect, 103);
    table[104] = crc16(poly, reflect, 104);
    table[105] = crc16(poly, reflect, 105);
    table[106] = crc16(poly, reflect, 106);
    table[107] = crc16(poly, reflect, 107);
    table[108] = crc16(poly, reflect, 108);
    table[109] = crc16(poly, reflect, 109);
    table[110] = crc16(poly, reflect, 110);
    table[111] = crc16(poly, reflect, 111);
    table[112] = crc16(poly, reflect, 112);
    table[113] = crc16(poly, reflect, 113);
    table[114] = crc16(poly, reflect, 114);
    table[115] = crc16(poly, reflect, 115);
    table[116] = crc16(poly, reflect, 116);
    table[117] = crc16(poly, reflect, 117);
    table[118] = crc16(poly, reflect, 118);
    table[119] = crc16(poly, reflect, 119);
    table[120] = crc16(poly, reflect, 120);
    table[121] = crc16(poly, reflect, 121);
    table[122] = crc16(poly, reflect, 122);
    table[123] = crc16(poly, reflect, 123);
    table[124] = crc16(poly, reflect, 124);
    table[125] = crc16(poly, reflect, 125);
    table[126] = crc16(poly, reflect, 126);
    table[127] = crc16(poly, reflect, 127);
    table[128] = crc16(poly, reflect, 128);
    table[129] = crc16(poly, reflect, 129);
    table[130] = crc16(poly, reflect, 130);
    table[131] = crc16(poly, reflect, 131);
    table[132] = crc16(poly, reflect, 132);
    table[133] = crc16(poly, reflect, 133);
    table[134] = crc16(poly, reflect, 134);
    table[135] = crc16(poly, reflect, 135);
    table[136] = crc16(poly, reflect, 136);
    table[137] = crc16(poly, reflect, 137);
    table[138] = crc16(poly, reflect, 138);
    table[139] = crc16(poly, reflect, 139);
    table[140] = crc16(poly, reflect, 140);
    table[141] = crc16(poly, reflect, 141);
    table[142] = crc16(poly, reflect, 142);
    table[143] = crc16(poly, reflect, 143);
    table[144] = crc16(poly, reflect, 144);
    table[145] = crc16(poly, reflect, 145);
    table[146] = crc16(poly, reflect, 146);
    table[147] = crc16(poly, reflect, 147);
    table[148] = crc16(poly, reflect, 148);
    table[149] = crc16(poly, reflect, 149);
    table[150] = crc16(poly, reflect, 150);
    table[151] = crc16(poly, reflect, 151);
    table[152] = crc16(poly, reflect, 152);
    table[153] = crc16(poly, reflect, 153);
    table[154] = crc16(poly, reflect, 154);
    table[155] = crc16(poly, reflect, 155);
    table[156] = crc16(poly, reflect, 156);
    table[157] = crc16(poly, reflect, 157);
    table[158] = crc16(poly, reflect, 158);
    table[159] = crc16(poly, reflect, 159);
    table[160] = crc16(poly, reflect, 160);
    table[161] = crc16(poly, reflect, 161);
    table[162] = crc16(poly, reflect, 162);
    table[163] = crc16(poly, reflect, 163);
    table[164] = crc16(poly, reflect, 164);
    table[165] = crc16(poly, reflect, 165);
    table[166] = crc16(poly, reflect, 166);
    table[167] = crc16(poly, reflect, 167);
    table[168] = crc16(poly, reflect, 168);
    table[169] = crc16(poly, reflect, 169);
    table[170] = crc16(poly, reflect, 170);
    table[171] = crc16(poly, reflect, 171);
    table[172] = crc16(poly, reflect, 172);
    table[173] = crc16(poly, reflect, 173);
    table[174] = crc16(poly, reflect, 174);
    table[175] = crc16(poly, reflect, 175);
    table[176] = crc16(poly, reflect, 176);
    table[177] = crc16(poly, reflect, 177);
    table[178] = crc16(poly, reflect, 178);
    table[179] = crc16(poly, reflect, 179);
    table[180] = crc16(poly, reflect, 180);
    table[181] = crc16(poly, reflect, 181);
    table[182] = crc16(poly, reflect, 182);
    table[183] = crc16(poly, reflect, 183);
    table[184] = crc16(poly, reflect, 184);
    table[185] = crc16(poly, reflect, 185);
    table[186] = crc16(poly, reflect, 186);
    table[187] = crc16(poly, reflect, 187);
    table[188] = crc16(poly, reflect, 188);
    table[189] = crc16(poly, reflect, 189);
    table[190] = crc16(poly, reflect, 190);
    table[191] = crc16(poly, reflect, 191);
    table[192] = crc16(poly, reflect, 192);
    table[193] = crc16(poly, reflect, 193);
    table[194] = crc16(poly, reflect, 194);
    table[195] = crc16(poly, reflect, 195);
    table[196] = crc16(poly, reflect, 196);
    table[197] = crc16(poly, reflect, 197);
    table[198] = crc16(poly, reflect, 198);
    table[199] = crc16(poly, reflect, 199);
    table[200] = crc16(poly, reflect, 200);
    table[201] = crc16(poly, reflect, 201);
    table[202] = crc16(poly, reflect, 202);
    table[203] = crc16(poly, reflect, 203);
    table[204] = crc16(poly, reflect, 204);
    table[205] = crc16(poly, reflect, 205);
    table[206] = crc16(poly, reflect, 206);
    table[207] = crc16(poly, reflect, 207);
    table[208] = crc16(poly, reflect, 208);
    table[209] = crc16(poly, reflect, 209);
    table[210] = crc16(poly, reflect, 210);
    table[211] = crc16(poly, reflect, 211);
    table[212] = crc16(poly, reflect, 212);
    table[213] = crc16(poly, reflect, 213);
    table[214] = crc16(poly, reflect, 214);
    table[215] = crc16(poly, reflect, 215);
    table[216] = crc16(poly, reflect, 216);
    table[217] = crc16(poly, reflect, 217);
    table[218] = crc16(poly, reflect, 218);
    table[219] = crc16(poly, reflect, 219);
    table[220] = crc16(poly, reflect, 220);
    table[221] = crc16(poly, reflect, 221);
    table[222] = crc16(poly, reflect, 222);
    table[223] = crc16(poly, reflect, 223);
    table[224] = crc16(poly, reflect, 224);
    table[225] = crc16(poly, reflect, 225);
    table[226] = crc16(poly, reflect, 226);
    table[227] = crc16(poly, reflect, 227);
    table[228] = crc16(poly, reflect, 228);
    table[229] = crc16(poly, reflect, 229);
    table[230] = crc16(poly, reflect, 230);
    table[231] = crc16(poly, reflect, 231);
    table[232] = crc16(poly, reflect, 232);
    table[233] = crc16(poly, reflect, 233);
    table[234] = crc16(poly, reflect, 234);
    table[235] = crc16(poly, reflect, 235);
    table[236] = crc16(poly, reflect, 236);
    table[237] = crc16(poly, reflect, 237);
    table[238] = crc16(poly, reflect, 238);
    table[239] = crc16(poly, reflect, 239);
    table[240] = crc16(poly, reflect, 240);
    table[241] = crc16(poly, reflect, 241);
    table[242] = crc16(poly, reflect, 242);
    table[243] = crc16(poly, reflect, 243);
    table[244] = crc16(poly, reflect, 244);
    table[245] = crc16(poly, reflect, 245);
    table[246] = crc16(poly, reflect, 246);
    table[247] = crc16(poly, reflect, 247);
    table[248] = crc16(poly, reflect, 248);
    table[249] = crc16(poly, reflect, 249);
    table[250] = crc16(poly, reflect, 250);
    table[251] = crc16(poly, reflect, 251);
    table[252] = crc16(poly, reflect, 252);
    table[253] = crc16(poly, reflect, 253);
    table[254] = crc16(poly, reflect, 254);
    table[255] = crc16(poly, reflect, 255);
    table
}
pub(crate) const fn crc32_table(poly: u32, reflect: bool) -> [u32; 256] {
    let mut table = [0u32; 256];
    table[0] = crc32(poly, reflect, 0);
    table[1] = crc32(poly, reflect, 1);
    table[2] = crc32(poly, reflect, 2);
    table[3] = crc32(poly, reflect, 3);
    table[4] = crc32(poly, reflect, 4);
    table[5] = crc32(poly, reflect, 5);
    table[6] = crc32(poly, reflect, 6);
    table[7] = crc32(poly, reflect, 7);
    table[8] = crc32(poly, reflect, 8);
    table[9] = crc32(poly, reflect, 9);
    table[10] = crc32(poly, reflect, 10);
    table[11] = crc32(poly, reflect, 11);
    table[12] = crc32(poly, reflect, 12);
    table[13] = crc32(poly, reflect, 13);
    table[14] = crc32(poly, reflect, 14);
    table[15] = crc32(poly, reflect, 15);
    table[16] = crc32(poly, reflect, 16);
    table[17] = crc32(poly, reflect, 17);
    table[18] = crc32(poly, reflect, 18);
    table[19] = crc32(poly, reflect, 19);
    table[20] = crc32(poly, reflect, 20);
    table[21] = crc32(poly, reflect, 21);
    table[22] = crc32(poly, reflect, 22);
    table[23] = crc32(poly, reflect, 23);
    table[24] = crc32(poly, reflect, 24);
    table[25] = crc32(poly, reflect, 25);
    table[26] = crc32(poly, reflect, 26);
    table[27] = crc32(poly, reflect, 27);
    table[28] = crc32(poly, reflect, 28);
    table[29] = crc32(poly, reflect, 29);
    table[30] = crc32(poly, reflect, 30);
    table[31] = crc32(poly, reflect, 31);
    table[32] = crc32(poly, reflect, 32);
    table[33] = crc32(poly, reflect, 33);
    table[34] = crc32(poly, reflect, 34);
    table[35] = crc32(poly, reflect, 35);
    table[36] = crc32(poly, reflect, 36);
    table[37] = crc32(poly, reflect, 37);
    table[38] = crc32(poly, reflect, 38);
    table[39] = crc32(poly, reflect, 39);
    table[40] = crc32(poly, reflect, 40);
    table[41] = crc32(poly, reflect, 41);
    table[42] = crc32(poly, reflect, 42);
    table[43] = crc32(poly, reflect, 43);
    table[44] = crc32(poly, reflect, 44);
    table[45] = crc32(poly, reflect, 45);
    table[46] = crc32(poly, reflect, 46);
    table[47] = crc32(poly, reflect, 47);
    table[48] = crc32(poly, reflect, 48);
    table[49] = crc32(poly, reflect, 49);
    table[50] = crc32(poly, reflect, 50);
    table[51] = crc32(poly, reflect, 51);
    table[52] = crc32(poly, reflect, 52);
    table[53] = crc32(poly, reflect, 53);
    table[54] = crc32(poly, reflect, 54);
    table[55] = crc32(poly, reflect, 55);
    table[56] = crc32(poly, reflect, 56);
    table[57] = crc32(poly, reflect, 57);
    table[58] = crc32(poly, reflect, 58);
    table[59] = crc32(poly, reflect, 59);
    table[60] = crc32(poly, reflect, 60);
    table[61] = crc32(poly, reflect, 61);
    table[62] = crc32(poly, reflect, 62);
    table[63] = crc32(poly, reflect, 63);
    table[64] = crc32(poly, reflect, 64);
    table[65] = crc32(poly, reflect, 65);
    table[66] = crc32(poly, reflect, 66);
    table[67] = crc32(poly, reflect, 67);
    table[68] = crc32(poly, reflect, 68);
    table[69] = crc32(poly, reflect, 69);
    table[70] = crc32(poly, reflect, 70);
    table[71] = crc32(poly, reflect, 71);
    table[72] = crc32(poly, reflect, 72);
    table[73] = crc32(poly, reflect, 73);
    table[74] = crc32(poly, reflect, 74);
    table[75] = crc32(poly, reflect, 75);
    table[76] = crc32(poly, reflect, 76);
    table[77] = crc32(poly, reflect, 77);
    table[78] = crc32(poly, reflect, 78);
    table[79] = crc32(poly, reflect, 79);
    table[80] = crc32(poly, reflect, 80);
    table[81] = crc32(poly, reflect, 81);
    table[82] = crc32(poly, reflect, 82);
    table[83] = crc32(poly, reflect, 83);
    table[84] = crc32(poly, reflect, 84);
    table[85] = crc32(poly, reflect, 85);
    table[86] = crc32(poly, reflect, 86);
    table[87] = crc32(poly, reflect, 87);
    table[88] = crc32(poly, reflect, 88);
    table[89] = crc32(poly, reflect, 89);
    table[90] = crc32(poly, reflect, 90);
    table[91] = crc32(poly, reflect, 91);
    table[92] = crc32(poly, reflect, 92);
    table[93] = crc32(poly, reflect, 93);
    table[94] = crc32(poly, reflect, 94);
    table[95] = crc32(poly, reflect, 95);
    table[96] = crc32(poly, reflect, 96);
    table[97] = crc32(poly, reflect, 97);
    table[98] = crc32(poly, reflect, 98);
    table[99] = crc32(poly, reflect, 99);
    table[100] = crc32(poly, reflect, 100);
    table[101] = crc32(poly, reflect, 101);
    table[102] = crc32(poly, reflect, 102);
    table[103] = crc32(poly, reflect, 103);
    table[104] = crc32(poly, reflect, 104);
    table[105] = crc32(poly, reflect, 105);
    table[106] = crc32(poly, reflect, 106);
    table[107] = crc32(poly, reflect, 107);
    table[108] = crc32(poly, reflect, 108);
    table[109] = crc32(poly, reflect, 109);
    table[110] = crc32(poly, reflect, 110);
    table[111] = crc32(poly, reflect, 111);
    table[112] = crc32(poly, reflect, 112);
    table[113] = crc32(poly, reflect, 113);
    table[114] = crc32(poly, reflect, 114);
    table[115] = crc32(poly, reflect, 115);
    table[116] = crc32(poly, reflect, 116);
    table[117] = crc32(poly, reflect, 117);
    table[118] = crc32(poly, reflect, 118);
    table[119] = crc32(poly, reflect, 119);
    table[120] = crc32(poly, reflect, 120);
    table[121] = crc32(poly, reflect, 121);
    table[122] = crc32(poly, reflect, 122);
    table[123] = crc32(poly, reflect, 123);
    table[124] = crc32(poly, reflect, 124);
    table[125] = crc32(poly, reflect, 125);
    table[126] = crc32(poly, reflect, 126);
    table[127] = crc32(poly, reflect, 127);
    table[128] = crc32(poly, reflect, 128);
    table[129] = crc32(poly, reflect, 129);
    table[130] = crc32(poly, reflect, 130);
    table[131] = crc32(poly, reflect, 131);
    table[132] = crc32(poly, reflect, 132);
    table[133] = crc32(poly, reflect, 133);
    table[134] = crc32(poly, reflect, 134);
    table[135] = crc32(poly, reflect, 135);
    table[136] = crc32(poly, reflect, 136);
    table[137] = crc32(poly, reflect, 137);
    table[138] = crc32(poly, reflect, 138);
    table[139] = crc32(poly, reflect, 139);
    table[140] = crc32(poly, reflect, 140);
    table[141] = crc32(poly, reflect, 141);
    table[142] = crc32(poly, reflect, 142);
    table[143] = crc32(poly, reflect, 143);
    table[144] = crc32(poly, reflect, 144);
    table[145] = crc32(poly, reflect, 145);
    table[146] = crc32(poly, reflect, 146);
    table[147] = crc32(poly, reflect, 147);
    table[148] = crc32(poly, reflect, 148);
    table[149] = crc32(poly, reflect, 149);
    table[150] = crc32(poly, reflect, 150);
    table[151] = crc32(poly, reflect, 151);
    table[152] = crc32(poly, reflect, 152);
    table[153] = crc32(poly, reflect, 153);
    table[154] = crc32(poly, reflect, 154);
    table[155] = crc32(poly, reflect, 155);
    table[156] = crc32(poly, reflect, 156);
    table[157] = crc32(poly, reflect, 157);
    table[158] = crc32(poly, reflect, 158);
    table[159] = crc32(poly, reflect, 159);
    table[160] = crc32(poly, reflect, 160);
    table[161] = crc32(poly, reflect, 161);
    table[162] = crc32(poly, reflect, 162);
    table[163] = crc32(poly, reflect, 163);
    table[164] = crc32(poly, reflect, 164);
    table[165] = crc32(poly, reflect, 165);
    table[166] = crc32(poly, reflect, 166);
    table[167] = crc32(poly, reflect, 167);
    table[168] = crc32(poly, reflect, 168);
    table[169] = crc32(poly, reflect, 169);
    table[170] = crc32(poly, reflect, 170);
    table[171] = crc32(poly, reflect, 171);
    table[172] = crc32(poly, reflect, 172);
    table[173] = crc32(poly, reflect, 173);
    table[174] = crc32(poly, reflect, 174);
    table[175] = crc32(poly, reflect, 175);
    table[176] = crc32(poly, reflect, 176);
    table[177] = crc32(poly, reflect, 177);
    table[178] = crc32(poly, reflect, 178);
    table[179] = crc32(poly, reflect, 179);
    table[180] = crc32(poly, reflect, 180);
    table[181] = crc32(poly, reflect, 181);
    table[182] = crc32(poly, reflect, 182);
    table[183] = crc32(poly, reflect, 183);
    table[184] = crc32(poly, reflect, 184);
    table[185] = crc32(poly, reflect, 185);
    table[186] = crc32(poly, reflect, 186);
    table[187] = crc32(poly, reflect, 187);
    table[188] = crc32(poly, reflect, 188);
    table[189] = crc32(poly, reflect, 189);
    table[190] = crc32(poly, reflect, 190);
    table[191] = crc32(poly, reflect, 191);
    table[192] = crc32(poly, reflect, 192);
    table[193] = crc32(poly, reflect, 193);
    table[194] = crc32(poly, reflect, 194);
    table[195] = crc32(poly, reflect, 195);
    table[196] = crc32(poly, reflect, 196);
    table[197] = crc32(poly, reflect, 197);
    table[198] = crc32(poly, reflect, 198);
    table[199] = crc32(poly, reflect, 199);
    table[200] = crc32(poly, reflect, 200);
    table[201] = crc32(poly, reflect, 201);
    table[202] = crc32(poly, reflect, 202);
    table[203] = crc32(poly, reflect, 203);
    table[204] = crc32(poly, reflect, 204);
    table[205] = crc32(poly, reflect, 205);
    table[206] = crc32(poly, reflect, 206);
    table[207] = crc32(poly, reflect, 207);
    table[208] = crc32(poly, reflect, 208);
    table[209] = crc32(poly, reflect, 209);
    table[210] = crc32(poly, reflect, 210);
    table[211] = crc32(poly, reflect, 211);
    table[212] = crc32(poly, reflect, 212);
    table[213] = crc32(poly, reflect, 213);
    table[214] = crc32(poly, reflect, 214);
    table[215] = crc32(poly, reflect, 215);
    table[216] = crc32(poly, reflect, 216);
    table[217] = crc32(poly, reflect, 217);
    table[218] = crc32(poly, reflect, 218);
    table[219] = crc32(poly, reflect, 219);
    table[220] = crc32(poly, reflect, 220);
    table[221] = crc32(poly, reflect, 221);
    table[222] = crc32(poly, reflect, 222);
    table[223] = crc32(poly, reflect, 223);
    table[224] = crc32(poly, reflect, 224);
    table[225] = crc32(poly, reflect, 225);
    table[226] = crc32(poly, reflect, 226);
    table[227] = crc32(poly, reflect, 227);
    table[228] = crc32(poly, reflect, 228);
    table[229] = crc32(poly, reflect, 229);
    table[230] = crc32(poly, reflect, 230);
    table[231] = crc32(poly, reflect, 231);
    table[232] = crc32(poly, reflect, 232);
    table[233] = crc32(poly, reflect, 233);
    table[234] = crc32(poly, reflect, 234);
    table[235] = crc32(poly, reflect, 235);
    table[236] = crc32(poly, reflect, 236);
    table[237] = crc32(poly, reflect, 237);
    table[238] = crc32(poly, reflect, 238);
    table[239] = crc32(poly, reflect, 239);
    table[240] = crc32(poly, reflect, 240);
    table[241] = crc32(poly, reflect, 241);
    table[242] = crc32(poly, reflect, 242);
    table[243] = crc32(poly, reflect, 243);
    table[244] = crc32(poly, reflect, 244);
    table[245] = crc32(poly, reflect, 245);
    table[246] = crc32(poly, reflect, 246);
    table[247] = crc32(poly, reflect, 247);
    table[248] = crc32(poly, reflect, 248);
    table[249] = crc32(poly, reflect, 249);
    table[250] = crc32(poly, reflect, 250);
    table[251] = crc32(poly, reflect, 251);
    table[252] = crc32(poly, reflect, 252);
    table[253] = crc32(poly, reflect, 253);
    table[254] = crc32(poly, reflect, 254);
    table[255] = crc32(poly, reflect, 255);
    table
}
pub(crate) const fn crc64_table(poly: u64, reflect: bool) -> [u64; 256] {
    let mut table = [0u64; 256];
    table[0] = crc64(poly, reflect, 0);
    table[1] = crc64(poly, reflect, 1);
    table[2] = crc64(poly, reflect, 2);
    table[3] = crc64(poly, reflect, 3);
    table[4] = crc64(poly, reflect, 4);
    table[5] = crc64(poly, reflect, 5);
    table[6] = crc64(poly, reflect, 6);
    table[7] = crc64(poly, reflect, 7);
    table[8] = crc64(poly, reflect, 8);
    table[9] = crc64(poly, reflect, 9);
    table[10] = crc64(poly, reflect, 10);
    table[11] = crc64(poly, reflect, 11);
    table[12] = crc64(poly, reflect, 12);
    table[13] = crc64(poly, reflect, 13);
    table[14] = crc64(poly, reflect, 14);
    table[15] = crc64(poly, reflect, 15);
    table[16] = crc64(poly, reflect, 16);
    table[17] = crc64(poly, reflect, 17);
    table[18] = crc64(poly, reflect, 18);
    table[19] = crc64(poly, reflect, 19);
    table[20] = crc64(poly, reflect, 20);
    table[21] = crc64(poly, reflect, 21);
    table[22] = crc64(poly, reflect, 22);
    table[23] = crc64(poly, reflect, 23);
    table[24] = crc64(poly, reflect, 24);
    table[25] = crc64(poly, reflect, 25);
    table[26] = crc64(poly, reflect, 26);
    table[27] = crc64(poly, reflect, 27);
    table[28] = crc64(poly, reflect, 28);
    table[29] = crc64(poly, reflect, 29);
    table[30] = crc64(poly, reflect, 30);
    table[31] = crc64(poly, reflect, 31);
    table[32] = crc64(poly, reflect, 32);
    table[33] = crc64(poly, reflect, 33);
    table[34] = crc64(poly, reflect, 34);
    table[35] = crc64(poly, reflect, 35);
    table[36] = crc64(poly, reflect, 36);
    table[37] = crc64(poly, reflect, 37);
    table[38] = crc64(poly, reflect, 38);
    table[39] = crc64(poly, reflect, 39);
    table[40] = crc64(poly, reflect, 40);
    table[41] = crc64(poly, reflect, 41);
    table[42] = crc64(poly, reflect, 42);
    table[43] = crc64(poly, reflect, 43);
    table[44] = crc64(poly, reflect, 44);
    table[45] = crc64(poly, reflect, 45);
    table[46] = crc64(poly, reflect, 46);
    table[47] = crc64(poly, reflect, 47);
    table[48] = crc64(poly, reflect, 48);
    table[49] = crc64(poly, reflect, 49);
    table[50] = crc64(poly, reflect, 50);
    table[51] = crc64(poly, reflect, 51);
    table[52] = crc64(poly, reflect, 52);
    table[53] = crc64(poly, reflect, 53);
    table[54] = crc64(poly, reflect, 54);
    table[55] = crc64(poly, reflect, 55);
    table[56] = crc64(poly, reflect, 56);
    table[57] = crc64(poly, reflect, 57);
    table[58] = crc64(poly, reflect, 58);
    table[59] = crc64(poly, reflect, 59);
    table[60] = crc64(poly, reflect, 60);
    table[61] = crc64(poly, reflect, 61);
    table[62] = crc64(poly, reflect, 62);
    table[63] = crc64(poly, reflect, 63);
    table[64] = crc64(poly, reflect, 64);
    table[65] = crc64(poly, reflect, 65);
    table[66] = crc64(poly, reflect, 66);
    table[67] = crc64(poly, reflect, 67);
    table[68] = crc64(poly, reflect, 68);
    table[69] = crc64(poly, reflect, 69);
    table[70] = crc64(poly, reflect, 70);
    table[71] = crc64(poly, reflect, 71);
    table[72] = crc64(poly, reflect, 72);
    table[73] = crc64(poly, reflect, 73);
    table[74] = crc64(poly, reflect, 74);
    table[75] = crc64(poly, reflect, 75);
    table[76] = crc64(poly, reflect, 76);
    table[77] = crc64(poly, reflect, 77);
    table[78] = crc64(poly, reflect, 78);
    table[79] = crc64(poly, reflect, 79);
    table[80] = crc64(poly, reflect, 80);
    table[81] = crc64(poly, reflect, 81);
    table[82] = crc64(poly, reflect, 82);
    table[83] = crc64(poly, reflect, 83);
    table[84] = crc64(poly, reflect, 84);
    table[85] = crc64(poly, reflect, 85);
    table[86] = crc64(poly, reflect, 86);
    table[87] = crc64(poly, reflect, 87);
    table[88] = crc64(poly, reflect, 88);
    table[89] = crc64(poly, reflect, 89);
    table[90] = crc64(poly, reflect, 90);
    table[91] = crc64(poly, reflect, 91);
    table[92] = crc64(poly, reflect, 92);
    table[93] = crc64(poly, reflect, 93);
    table[94] = crc64(poly, reflect, 94);
    table[95] = crc64(poly, reflect, 95);
    table[96] = crc64(poly, reflect, 96);
    table[97] = crc64(poly, reflect, 97);
    table[98] = crc64(poly, reflect, 98);
    table[99] = crc64(poly, reflect, 99);
    table[100] = crc64(poly, reflect, 100);
    table[101] = crc64(poly, reflect, 101);
    table[102] = crc64(poly, reflect, 102);
    table[103] = crc64(poly, reflect, 103);
    table[104] = crc64(poly, reflect, 104);
    table[105] = crc64(poly, reflect, 105);
    table[106] = crc64(poly, reflect, 106);
    table[107] = crc64(poly, reflect, 107);
    table[108] = crc64(poly, reflect, 108);
    table[109] = crc64(poly, reflect, 109);
    table[110] = crc64(poly, reflect, 110);
    table[111] = crc64(poly, reflect, 111);
    table[112] = crc64(poly, reflect, 112);
    table[113] = crc64(poly, reflect, 113);
    table[114] = crc64(poly, reflect, 114);
    table[115] = crc64(poly, reflect, 115);
    table[116] = crc64(poly, reflect, 116);
    table[117] = crc64(poly, reflect, 117);
    table[118] = crc64(poly, reflect, 118);
    table[119] = crc64(poly, reflect, 119);
    table[120] = crc64(poly, reflect, 120);
    table[121] = crc64(poly, reflect, 121);
    table[122] = crc64(poly, reflect, 122);
    table[123] = crc64(poly, reflect, 123);
    table[124] = crc64(poly, reflect, 124);
    table[125] = crc64(poly, reflect, 125);
    table[126] = crc64(poly, reflect, 126);
    table[127] = crc64(poly, reflect, 127);
    table[128] = crc64(poly, reflect, 128);
    table[129] = crc64(poly, reflect, 129);
    table[130] = crc64(poly, reflect, 130);
    table[131] = crc64(poly, reflect, 131);
    table[132] = crc64(poly, reflect, 132);
    table[133] = crc64(poly, reflect, 133);
    table[134] = crc64(poly, reflect, 134);
    table[135] = crc64(poly, reflect, 135);
    table[136] = crc64(poly, reflect, 136);
    table[137] = crc64(poly, reflect, 137);
    table[138] = crc64(poly, reflect, 138);
    table[139] = crc64(poly, reflect, 139);
    table[140] = crc64(poly, reflect, 140);
    table[141] = crc64(poly, reflect, 141);
    table[142] = crc64(poly, reflect, 142);
    table[143] = crc64(poly, reflect, 143);
    table[144] = crc64(poly, reflect, 144);
    table[145] = crc64(poly, reflect, 145);
    table[146] = crc64(poly, reflect, 146);
    table[147] = crc64(poly, reflect, 147);
    table[148] = crc64(poly, reflect, 148);
    table[149] = crc64(poly, reflect, 149);
    table[150] = crc64(poly, reflect, 150);
    table[151] = crc64(poly, reflect, 151);
    table[152] = crc64(poly, reflect, 152);
    table[153] = crc64(poly, reflect, 153);
    table[154] = crc64(poly, reflect, 154);
    table[155] = crc64(poly, reflect, 155);
    table[156] = crc64(poly, reflect, 156);
    table[157] = crc64(poly, reflect, 157);
    table[158] = crc64(poly, reflect, 158);
    table[159] = crc64(poly, reflect, 159);
    table[160] = crc64(poly, reflect, 160);
    table[161] = crc64(poly, reflect, 161);
    table[162] = crc64(poly, reflect, 162);
    table[163] = crc64(poly, reflect, 163);
    table[164] = crc64(poly, reflect, 164);
    table[165] = crc64(poly, reflect, 165);
    table[166] = crc64(poly, reflect, 166);
    table[167] = crc64(poly, reflect, 167);
    table[168] = crc64(poly, reflect, 168);
    table[169] = crc64(poly, reflect, 169);
    table[170] = crc64(poly, reflect, 170);
    table[171] = crc64(poly, reflect, 171);
    table[172] = crc64(poly, reflect, 172);
    table[173] = crc64(poly, reflect, 173);
    table[174] = crc64(poly, reflect, 174);
    table[175] = crc64(poly, reflect, 175);
    table[176] = crc64(poly, reflect, 176);
    table[177] = crc64(poly, reflect, 177);
    table[178] = crc64(poly, reflect, 178);
    table[179] = crc64(poly, reflect, 179);
    table[180] = crc64(poly, reflect, 180);
    table[181] = crc64(poly, reflect, 181);
    table[182] = crc64(poly, reflect, 182);
    table[183] = crc64(poly, reflect, 183);
    table[184] = crc64(poly, reflect, 184);
    table[185] = crc64(poly, reflect, 185);
    table[186] = crc64(poly, reflect, 186);
    table[187] = crc64(poly, reflect, 187);
    table[188] = crc64(poly, reflect, 188);
    table[189] = crc64(poly, reflect, 189);
    table[190] = crc64(poly, reflect, 190);
    table[191] = crc64(poly, reflect, 191);
    table[192] = crc64(poly, reflect, 192);
    table[193] = crc64(poly, reflect, 193);
    table[194] = crc64(poly, reflect, 194);
    table[195] = crc64(poly, reflect, 195);
    table[196] = crc64(poly, reflect, 196);
    table[197] = crc64(poly, reflect, 197);
    table[198] = crc64(poly, reflect, 198);
    table[199] = crc64(poly, reflect, 199);
    table[200] = crc64(poly, reflect, 200);
    table[201] = crc64(poly, reflect, 201);
    table[202] = crc64(poly, reflect, 202);
    table[203] = crc64(poly, reflect, 203);
    table[204] = crc64(poly, reflect, 204);
    table[205] = crc64(poly, reflect, 205);
    table[206] = crc64(poly, reflect, 206);
    table[207] = crc64(poly, reflect, 207);
    table[208] = crc64(poly, reflect, 208);
    table[209] = crc64(poly, reflect, 209);
    table[210] = crc64(poly, reflect, 210);
    table[211] = crc64(poly, reflect, 211);
    table[212] = crc64(poly, reflect, 212);
    table[213] = crc64(poly, reflect, 213);
    table[214] = crc64(poly, reflect, 214);
    table[215] = crc64(poly, reflect, 215);
    table[216] = crc64(poly, reflect, 216);
    table[217] = crc64(poly, reflect, 217);
    table[218] = crc64(poly, reflect, 218);
    table[219] = crc64(poly, reflect, 219);
    table[220] = crc64(poly, reflect, 220);
    table[221] = crc64(poly, reflect, 221);
    table[222] = crc64(poly, reflect, 222);
    table[223] = crc64(poly, reflect, 223);
    table[224] = crc64(poly, reflect, 224);
    table[225] = crc64(poly, reflect, 225);
    table[226] = crc64(poly, reflect, 226);
    table[227] = crc64(poly, reflect, 227);
    table[228] = crc64(poly, reflect, 228);
    table[229] = crc64(poly, reflect, 229);
    table[230] = crc64(poly, reflect, 230);
    table[231] = crc64(poly, reflect, 231);
    table[232] = crc64(poly, reflect, 232);
    table[233] = crc64(poly, reflect, 233);
    table[234] = crc64(poly, reflect, 234);
    table[235] = crc64(poly, reflect, 235);
    table[236] = crc64(poly, reflect, 236);
    table[237] = crc64(poly, reflect, 237);
    table[238] = crc64(poly, reflect, 238);
    table[239] = crc64(poly, reflect, 239);
    table[240] = crc64(poly, reflect, 240);
    table[241] = crc64(poly, reflect, 241);
    table[242] = crc64(poly, reflect, 242);
    table[243] = crc64(poly, reflect, 243);
    table[244] = crc64(poly, reflect, 244);
    table[245] = crc64(poly, reflect, 245);
    table[246] = crc64(poly, reflect, 246);
    table[247] = crc64(poly, reflect, 247);
    table[248] = crc64(poly, reflect, 248);
    table[249] = crc64(poly, reflect, 249);
    table[250] = crc64(poly, reflect, 250);
    table[251] = crc64(poly, reflect, 251);
    table[252] = crc64(poly, reflect, 252);
    table[253] = crc64(poly, reflect, 253);
    table[254] = crc64(poly, reflect, 254);
    table[255] = crc64(poly, reflect, 255);
    table
}
