// WARNING: this file was generated by scripts/gen-indic-table.py

#![allow(non_camel_case_types)]
#![allow(unused_imports)]

use super::ot_shaper_indic::indic_category_t::*;
use super::ot_shaper_indic::indic_position_t::*;

use OT_A     as ISC_A;          /*  51 chars; A */
use OT_C     as ISC_C;          /* 521 chars; C */
use OT_CM    as ISC_CM;         /*  10 chars; CM */
use OT_CS    as ISC_CS;         /*   2 chars; CS */
use OT_Coeng as ISC_Co;         /*   2 chars; Coeng */
use OT_DOTTEDCIRCLE as ISC_DC;         /*   1 chars; DOTTEDCIRCLE */
use OT_H     as ISC_H;          /*  10 chars; H */
use OT_M     as ISC_M;          /* 160 chars; M */
use OT_N     as ISC_N;          /*  35 chars; N */
use OT_PLACEHOLDER as ISC_GB;         /* 168 chars; PLACEHOLDER */
use OT_RS    as ISC_RS;         /*   2 chars; RS */
use OT_Ra    as ISC_R;          /*  11 chars; Ra */
use OT_Repha as ISC_Rf;         /*   1 chars; Repha */
use OT_SM    as ISC_SM;         /*  56 chars; SM */
use OT_Symbol as ISC_S;          /*  22 chars; Symbol */
use OT_V     as ISC_V;          /* 190 chars; V */
use OT_VAbv  as ISC_VA;         /*  22 chars; VAbv */
use OT_VBlw  as ISC_VB;         /*   7 chars; VBlw */
use OT_VPre  as ISC_VL;         /*   5 chars; VPre */
use OT_VPst  as ISC_VR;         /*  15 chars; VPst */
use OT_X     as ISC_X;          /*   2 chars; X */
use OT_ZWJ   as ISC_ZWJ;        /*   1 chars; ZWJ */
use OT_ZWNJ  as ISC_ZWNJ;       /*   1 chars; ZWNJ */

use POS_ABOVE_C as IMC_T;          /*  29 chars; ABOVE_C */
use POS_AFTER_MAIN as IMC_A;          /*   3 chars; AFTER_MAIN */
use POS_AFTER_POST as IMC_AP;         /*  50 chars; AFTER_POST */
use POS_AFTER_SUB as IMC_AS;         /*  60 chars; AFTER_SUB */
use POS_BASE_C as IMC_C;          /* 903 chars; BASE_C */
use POS_BEFORE_SUB as IMC_BS;         /*  31 chars; BEFORE_SUB */
use POS_BELOW_C as IMC_B;          /*  13 chars; BELOW_C */
use POS_END  as IMC_X;          /*  42 chars; END */
use POS_POST_C as IMC_R;          /*  15 chars; POST_C */
use POS_PRE_C as IMC_L;          /*   5 chars; PRE_C */
use POS_PRE_M as IMC_LM;         /*  16 chars; PRE_M */
use POS_SMVD as IMC_SM;         /* 128 chars; SMVD */


pub type SyllabicCategory = u8;
pub type MatraCategory = u8;

#[rustfmt::skip]
const TABLE: &[(SyllabicCategory, MatraCategory)] = &[


  /* Basic Latin */

  /* 0028 */  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X), (ISC_GB,IMC_C),  (ISC_X,IMC_X),  (ISC_X,IMC_X),
  /* 0030 */ (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C),
  /* 0038 */ (ISC_GB,IMC_C), (ISC_GB,IMC_C),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),

  /* Latin-1 Supplement */

  /* 00B0 */  (ISC_X,IMC_X),  (ISC_X,IMC_X),(ISC_SM,IMC_SM),(ISC_SM,IMC_SM),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),
  /* 00B8 */  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),
  /* 00C0 */  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),
  /* 00C8 */  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),
  /* 00D0 */  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X), (ISC_GB,IMC_C),

  /* Devanagari */

  /* 0900 */(ISC_SM,IMC_SM),(ISC_SM,IMC_SM),(ISC_SM,IMC_SM),(ISC_SM,IMC_SM),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),
  /* 0908 */  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),
  /* 0910 */  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* 0918 */  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* 0920 */  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* 0928 */  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* 0930 */  (ISC_R,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* 0938 */  (ISC_C,IMC_C),  (ISC_C,IMC_C), (ISC_M,IMC_AS), (ISC_M,IMC_AS),  (ISC_N,IMC_X), (ISC_S,IMC_SM), (ISC_M,IMC_AS), (ISC_M,IMC_LM),
  /* 0940 */ (ISC_M,IMC_AS), (ISC_M,IMC_AS), (ISC_M,IMC_AS), (ISC_M,IMC_AS), (ISC_M,IMC_AS), (ISC_M,IMC_AS), (ISC_M,IMC_AS), (ISC_M,IMC_AS),
  /* 0948 */ (ISC_M,IMC_AS), (ISC_M,IMC_AS), (ISC_M,IMC_AS), (ISC_M,IMC_AS), (ISC_M,IMC_AS),  (ISC_H,IMC_B), (ISC_M,IMC_LM), (ISC_M,IMC_AS),
  /* 0950 */  (ISC_X,IMC_X), (ISC_A,IMC_SM), (ISC_A,IMC_SM),(ISC_SM,IMC_SM),(ISC_SM,IMC_SM), (ISC_M,IMC_AS), (ISC_M,IMC_AS), (ISC_M,IMC_AS),
  /* 0958 */  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* 0960 */  (ISC_V,IMC_C),  (ISC_V,IMC_C), (ISC_M,IMC_AS), (ISC_M,IMC_AS),  (ISC_X,IMC_X),  (ISC_X,IMC_X), (ISC_GB,IMC_C), (ISC_GB,IMC_C),
  /* 0968 */ (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C),
  /* 0970 */  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),
  /* 0978 */  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),

  /* Bengali */

  /* 0980 */ (ISC_GB,IMC_C),(ISC_SM,IMC_SM),(ISC_SM,IMC_SM),(ISC_SM,IMC_SM),  (ISC_X,IMC_X),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),
  /* 0988 */  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_V,IMC_C),
  /* 0990 */  (ISC_V,IMC_C),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* 0998 */  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* 09A0 */  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* 09A8 */  (ISC_C,IMC_C),  (ISC_X,IMC_X),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* 09B0 */  (ISC_R,IMC_C),  (ISC_X,IMC_X),  (ISC_C,IMC_C),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* 09B8 */  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_N,IMC_X), (ISC_S,IMC_SM), (ISC_M,IMC_AP), (ISC_M,IMC_LM),
  /* 09C0 */ (ISC_M,IMC_AP), (ISC_M,IMC_AS), (ISC_M,IMC_AS), (ISC_M,IMC_AS), (ISC_M,IMC_AS),  (ISC_X,IMC_X),  (ISC_X,IMC_X), (ISC_M,IMC_LM),
  /* 09C8 */ (ISC_M,IMC_LM),  (ISC_X,IMC_X),  (ISC_X,IMC_X), (ISC_M,IMC_AP), (ISC_M,IMC_AP),  (ISC_H,IMC_B),  (ISC_C,IMC_C),  (ISC_X,IMC_X),
  /* 09D0 */  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X), (ISC_M,IMC_AP),
  /* 09D8 */  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_X,IMC_X),  (ISC_C,IMC_C),
  /* 09E0 */  (ISC_V,IMC_C),  (ISC_V,IMC_C), (ISC_M,IMC_AS), (ISC_M,IMC_AS),  (ISC_X,IMC_X),  (ISC_X,IMC_X), (ISC_GB,IMC_C), (ISC_GB,IMC_C),
  /* 09E8 */ (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C),
  /* 09F0 */  (ISC_R,IMC_C),  (ISC_C,IMC_C),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),
  /* 09F8 */  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X), (ISC_GB,IMC_C),  (ISC_X,IMC_X),(ISC_SM,IMC_SM),  (ISC_X,IMC_X),

  /* Gurmukhi */

  /* 0A00 */  (ISC_X,IMC_X),(ISC_SM,IMC_SM),(ISC_SM,IMC_SM),(ISC_SM,IMC_SM),  (ISC_X,IMC_X),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),
  /* 0A08 */  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_V,IMC_C),
  /* 0A10 */  (ISC_V,IMC_C),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* 0A18 */  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* 0A20 */  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* 0A28 */  (ISC_C,IMC_C),  (ISC_X,IMC_X),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* 0A30 */  (ISC_R,IMC_C),  (ISC_X,IMC_X),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_X,IMC_X),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_X,IMC_X),
  /* 0A38 */  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_N,IMC_X),  (ISC_X,IMC_X), (ISC_M,IMC_AP), (ISC_M,IMC_LM),
  /* 0A40 */ (ISC_M,IMC_AP), (ISC_M,IMC_AP), (ISC_M,IMC_AP),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X), (ISC_M,IMC_AP),
  /* 0A48 */ (ISC_M,IMC_AP),  (ISC_X,IMC_X),  (ISC_X,IMC_X), (ISC_M,IMC_AP), (ISC_M,IMC_AP),  (ISC_H,IMC_B),  (ISC_X,IMC_X),  (ISC_X,IMC_X),
  /* 0A50 */  (ISC_X,IMC_X),  (ISC_M,IMC_B),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),
  /* 0A58 */  (ISC_X,IMC_X),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_X,IMC_X),  (ISC_C,IMC_C),  (ISC_X,IMC_X),
  /* 0A60 */  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X), (ISC_GB,IMC_C), (ISC_GB,IMC_C),
  /* 0A68 */ (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C),
  /* 0A70 */(ISC_SM,IMC_SM),(ISC_SM,IMC_SM),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_X,IMC_X), (ISC_CM,IMC_C),  (ISC_X,IMC_X),  (ISC_X,IMC_X),
  /* 0A78 */  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),

  /* Gujarati */

  /* 0A80 */  (ISC_X,IMC_X),(ISC_SM,IMC_SM),(ISC_SM,IMC_SM),(ISC_SM,IMC_SM),  (ISC_X,IMC_X),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),
  /* 0A88 */  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_X,IMC_X),  (ISC_V,IMC_C),
  /* 0A90 */  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_X,IMC_X),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* 0A98 */  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* 0AA0 */  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* 0AA8 */  (ISC_C,IMC_C),  (ISC_X,IMC_X),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* 0AB0 */  (ISC_R,IMC_C),  (ISC_X,IMC_X),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_X,IMC_X),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* 0AB8 */  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_N,IMC_X), (ISC_S,IMC_SM), (ISC_M,IMC_AP), (ISC_M,IMC_LM),
  /* 0AC0 */ (ISC_M,IMC_AP), (ISC_M,IMC_AP), (ISC_M,IMC_AP), (ISC_M,IMC_AP), (ISC_M,IMC_AP), (ISC_M,IMC_AS),  (ISC_X,IMC_X), (ISC_M,IMC_AS),
  /* 0AC8 */ (ISC_M,IMC_AS), (ISC_M,IMC_AP),  (ISC_X,IMC_X), (ISC_M,IMC_AP), (ISC_M,IMC_AP),  (ISC_H,IMC_B),  (ISC_X,IMC_X),  (ISC_X,IMC_X),
  /* 0AD0 */  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),
  /* 0AD8 */  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),
  /* 0AE0 */  (ISC_V,IMC_C),  (ISC_V,IMC_C), (ISC_M,IMC_AP), (ISC_M,IMC_AP),  (ISC_X,IMC_X),  (ISC_X,IMC_X), (ISC_GB,IMC_C), (ISC_GB,IMC_C),
  /* 0AE8 */ (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C),
  /* 0AF0 */  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),
  /* 0AF8 */  (ISC_X,IMC_X),  (ISC_C,IMC_C), (ISC_A,IMC_SM),  (ISC_N,IMC_X), (ISC_A,IMC_SM),  (ISC_N,IMC_X),  (ISC_N,IMC_X),  (ISC_N,IMC_X),

  /* Oriya */

  /* 0B00 */  (ISC_X,IMC_X),(ISC_SM,IMC_BS),(ISC_SM,IMC_SM),(ISC_SM,IMC_SM),  (ISC_X,IMC_X),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),
  /* 0B08 */  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_V,IMC_C),
  /* 0B10 */  (ISC_V,IMC_C),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* 0B18 */  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* 0B20 */  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* 0B28 */  (ISC_C,IMC_C),  (ISC_X,IMC_X),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* 0B30 */  (ISC_R,IMC_C),  (ISC_X,IMC_X),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_X,IMC_X),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* 0B38 */  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_N,IMC_X), (ISC_S,IMC_SM), (ISC_M,IMC_AP),  (ISC_M,IMC_A),
  /* 0B40 */ (ISC_M,IMC_AP), (ISC_M,IMC_AS), (ISC_M,IMC_AS), (ISC_M,IMC_AS), (ISC_M,IMC_AS),  (ISC_X,IMC_X),  (ISC_X,IMC_X), (ISC_M,IMC_LM),
  /* 0B48 */  (ISC_M,IMC_A),  (ISC_X,IMC_X),  (ISC_X,IMC_X), (ISC_M,IMC_AP), (ISC_M,IMC_AP),  (ISC_H,IMC_B),  (ISC_X,IMC_X),  (ISC_X,IMC_X),
  /* 0B50 */  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_N,IMC_X),  (ISC_M,IMC_A), (ISC_M,IMC_AP),
  /* 0B58 */  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_X,IMC_X),  (ISC_C,IMC_C),
  /* 0B60 */  (ISC_V,IMC_C),  (ISC_V,IMC_C), (ISC_M,IMC_AS), (ISC_M,IMC_AS),  (ISC_X,IMC_X),  (ISC_X,IMC_X), (ISC_GB,IMC_C), (ISC_GB,IMC_C),
  /* 0B68 */ (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C),
  /* 0B70 */  (ISC_X,IMC_X),  (ISC_C,IMC_C),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),
  /* 0B78 */  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),

  /* Tamil */

  /* 0B80 */  (ISC_X,IMC_X),  (ISC_X,IMC_X),(ISC_SM,IMC_SM),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),
  /* 0B88 */  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_V,IMC_C),  (ISC_V,IMC_C),
  /* 0B90 */  (ISC_V,IMC_C),  (ISC_X,IMC_X),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_C,IMC_C),  (ISC_X,IMC_X),  (ISC_X,IMC_X),
  /* 0B98 */  (ISC_X,IMC_X),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_X,IMC_X),  (ISC_C,IMC_C),  (ISC_X,IMC_X),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* 0BA0 */  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),
  /* 0BA8 */  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* 0BB0 */  (ISC_R,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* 0BB8 */  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X), (ISC_M,IMC_AP), (ISC_M,IMC_AP),
  /* 0BC0 */ (ISC_M,IMC_AS), (ISC_M,IMC_AP), (ISC_M,IMC_AP),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X), (ISC_M,IMC_LM), (ISC_M,IMC_LM),
  /* 0BC8 */ (ISC_M,IMC_LM),  (ISC_X,IMC_X), (ISC_M,IMC_AP), (ISC_M,IMC_AP), (ISC_M,IMC_AP),  (ISC_H,IMC_T),  (ISC_X,IMC_X),  (ISC_X,IMC_X),
  /* 0BD0 */  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X), (ISC_M,IMC_AP),
  /* 0BD8 */  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),
  /* 0BE0 */  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X), (ISC_GB,IMC_C), (ISC_GB,IMC_C),
  /* 0BE8 */ (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C),
  /* 0BF0 */  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),
  /* 0BF8 */  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),

  /* Telugu */

  /* 0C00 */(ISC_SM,IMC_SM),(ISC_SM,IMC_SM),(ISC_SM,IMC_SM),(ISC_SM,IMC_SM),(ISC_SM,IMC_SM),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),
  /* 0C08 */  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_X,IMC_X),  (ISC_V,IMC_C),  (ISC_V,IMC_C),
  /* 0C10 */  (ISC_V,IMC_C),  (ISC_X,IMC_X),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* 0C18 */  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* 0C20 */  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* 0C28 */  (ISC_C,IMC_C),  (ISC_X,IMC_X),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* 0C30 */  (ISC_R,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* 0C38 */  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_N,IMC_X), (ISC_S,IMC_SM), (ISC_M,IMC_BS), (ISC_M,IMC_BS),
  /* 0C40 */ (ISC_M,IMC_BS), (ISC_M,IMC_AS), (ISC_M,IMC_AS), (ISC_M,IMC_AS), (ISC_M,IMC_AS),  (ISC_X,IMC_X), (ISC_M,IMC_BS), (ISC_M,IMC_BS),
  /* 0C48 */ (ISC_M,IMC_BS),  (ISC_X,IMC_X), (ISC_M,IMC_BS), (ISC_M,IMC_BS), (ISC_M,IMC_BS),  (ISC_H,IMC_T),  (ISC_X,IMC_X),  (ISC_X,IMC_X),
  /* 0C50 */  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X), (ISC_M,IMC_BS), (ISC_M,IMC_BS),  (ISC_X,IMC_X),
  /* 0C58 */  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_C,IMC_C),  (ISC_X,IMC_X),  (ISC_X,IMC_X),
  /* 0C60 */  (ISC_V,IMC_C),  (ISC_V,IMC_C), (ISC_M,IMC_BS), (ISC_M,IMC_BS),  (ISC_X,IMC_X),  (ISC_X,IMC_X), (ISC_GB,IMC_C), (ISC_GB,IMC_C),
  /* 0C68 */ (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C),
  /* 0C70 */  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),
  /* 0C78 */  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),

  /* Kannada */

  /* 0C80 */ (ISC_GB,IMC_C),(ISC_SM,IMC_SM),(ISC_SM,IMC_SM),(ISC_SM,IMC_SM),  (ISC_X,IMC_X),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),
  /* 0C88 */  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_X,IMC_X),  (ISC_V,IMC_C),  (ISC_V,IMC_C),
  /* 0C90 */  (ISC_V,IMC_C),  (ISC_X,IMC_X),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* 0C98 */  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* 0CA0 */  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* 0CA8 */  (ISC_C,IMC_C),  (ISC_X,IMC_X),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* 0CB0 */  (ISC_R,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_X,IMC_X),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* 0CB8 */  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_N,IMC_X), (ISC_S,IMC_SM), (ISC_M,IMC_BS), (ISC_M,IMC_BS),
  /* 0CC0 */ (ISC_M,IMC_BS), (ISC_M,IMC_BS), (ISC_M,IMC_BS), (ISC_M,IMC_BS), (ISC_M,IMC_BS),  (ISC_X,IMC_X), (ISC_M,IMC_BS), (ISC_M,IMC_BS),
  /* 0CC8 */ (ISC_M,IMC_BS),  (ISC_X,IMC_X), (ISC_M,IMC_BS), (ISC_M,IMC_BS), (ISC_M,IMC_BS),  (ISC_H,IMC_T),  (ISC_X,IMC_X),  (ISC_X,IMC_X),
  /* 0CD0 */  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X), (ISC_M,IMC_BS), (ISC_M,IMC_BS),  (ISC_X,IMC_X),
  /* 0CD8 */  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_X,IMC_X),
  /* 0CE0 */  (ISC_V,IMC_C),  (ISC_V,IMC_C), (ISC_M,IMC_BS), (ISC_M,IMC_BS),  (ISC_X,IMC_X),  (ISC_X,IMC_X), (ISC_GB,IMC_C), (ISC_GB,IMC_C),
  /* 0CE8 */ (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C),
  /* 0CF0 */  (ISC_X,IMC_X), (ISC_CS,IMC_C), (ISC_CS,IMC_C),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),
  /* 0CF8 */  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),

  /* Malayalam */

  /* 0D00 */(ISC_SM,IMC_SM),(ISC_SM,IMC_SM),(ISC_SM,IMC_SM),(ISC_SM,IMC_SM), (ISC_GB,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),
  /* 0D08 */  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_X,IMC_X),  (ISC_V,IMC_C),  (ISC_V,IMC_C),
  /* 0D10 */  (ISC_V,IMC_C),  (ISC_X,IMC_X),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* 0D18 */  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* 0D20 */  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* 0D28 */  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* 0D30 */  (ISC_R,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* 0D38 */  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C), (ISC_M,IMC_AS), (ISC_M,IMC_AS), (ISC_S,IMC_SM), (ISC_M,IMC_AP), (ISC_M,IMC_AP),
  /* 0D40 */ (ISC_M,IMC_AP), (ISC_M,IMC_AP), (ISC_M,IMC_AP), (ISC_M,IMC_AP), (ISC_M,IMC_AP),  (ISC_X,IMC_X), (ISC_M,IMC_LM), (ISC_M,IMC_LM),
  /* 0D48 */ (ISC_M,IMC_LM),  (ISC_X,IMC_X), (ISC_M,IMC_AP), (ISC_M,IMC_AP), (ISC_M,IMC_AP),  (ISC_H,IMC_T), (ISC_Rf,IMC_X),  (ISC_X,IMC_X),
  /* 0D50 */  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C), (ISC_M,IMC_AP),
  /* 0D58 */  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_V,IMC_C),
  /* 0D60 */  (ISC_V,IMC_C),  (ISC_V,IMC_C), (ISC_M,IMC_AP), (ISC_M,IMC_AP),  (ISC_X,IMC_X),  (ISC_X,IMC_X), (ISC_GB,IMC_C), (ISC_GB,IMC_C),
  /* 0D68 */ (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C),
  /* 0D70 */  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),
  /* 0D78 */  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),

  /* Sinhala */

  /* 0D80 */  (ISC_X,IMC_X),(ISC_SM,IMC_SM),(ISC_SM,IMC_SM),(ISC_SM,IMC_SM),  (ISC_X,IMC_X),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),
  /* 0D88 */  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),
  /* 0D90 */  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_X,IMC_X),
  /* 0D98 */  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* 0DA0 */  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* 0DA8 */  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* 0DB0 */  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_X,IMC_X),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* 0DB8 */  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_R,IMC_C),  (ISC_X,IMC_X),  (ISC_C,IMC_C),  (ISC_X,IMC_X),  (ISC_X,IMC_X),
  /* 0DC0 */  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_X,IMC_X),
  /* 0DC8 */  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_H,IMC_T),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X), (ISC_M,IMC_AS),
  /* 0DD0 */ (ISC_M,IMC_AS), (ISC_M,IMC_AS), (ISC_M,IMC_AS), (ISC_M,IMC_AS), (ISC_M,IMC_AS),  (ISC_X,IMC_X), (ISC_M,IMC_AS),  (ISC_X,IMC_X),
  /* 0DD8 */ (ISC_M,IMC_AS), (ISC_M,IMC_LM), (ISC_M,IMC_AS), (ISC_M,IMC_LM), (ISC_M,IMC_AS), (ISC_M,IMC_AS), (ISC_M,IMC_AS), (ISC_M,IMC_AS),
  /* 0DE0 */  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X), (ISC_GB,IMC_C), (ISC_GB,IMC_C),
  /* 0DE8 */ (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C),
  /* 0DF0 */  (ISC_X,IMC_X),  (ISC_X,IMC_X), (ISC_M,IMC_AS), (ISC_M,IMC_AS),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),

  /* Myanmar */

  /* 1000 */  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* 1008 */  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* 1010 */  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* 1018 */  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* 1020 */  (ISC_C,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),
  /* 1028 */  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C), (ISC_VR,IMC_R), (ISC_VR,IMC_R), (ISC_VA,IMC_T), (ISC_VA,IMC_T), (ISC_VB,IMC_B),
  /* 1030 */ (ISC_VB,IMC_B), (ISC_VL,IMC_L), (ISC_VA,IMC_T), (ISC_VA,IMC_T), (ISC_VA,IMC_T), (ISC_VA,IMC_T),(ISC_SM,IMC_SM),  (ISC_N,IMC_X),
  /* 1038 */(ISC_SM,IMC_SM), (ISC_Co,IMC_X), (ISC_VA,IMC_T), (ISC_CM,IMC_C), (ISC_CM,IMC_C), (ISC_CM,IMC_C), (ISC_CM,IMC_C),  (ISC_C,IMC_C),
  /* 1040 */ (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C),
  /* 1048 */ (ISC_GB,IMC_C), (ISC_GB,IMC_C),  (ISC_X,IMC_X), (ISC_GB,IMC_C),  (ISC_X,IMC_X),  (ISC_X,IMC_X), (ISC_GB,IMC_C),  (ISC_X,IMC_X),
  /* 1050 */  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C), (ISC_VR,IMC_R), (ISC_VR,IMC_R),
  /* 1058 */ (ISC_VB,IMC_B), (ISC_VB,IMC_B),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C), (ISC_CM,IMC_C), (ISC_CM,IMC_C),
  /* 1060 */ (ISC_CM,IMC_C),  (ISC_C,IMC_C), (ISC_VR,IMC_R),  (ISC_N,IMC_X),  (ISC_N,IMC_X),  (ISC_C,IMC_C),  (ISC_C,IMC_C), (ISC_VR,IMC_R),
  /* 1068 */ (ISC_VR,IMC_R),  (ISC_N,IMC_X),  (ISC_N,IMC_X),  (ISC_N,IMC_X),  (ISC_N,IMC_X),  (ISC_N,IMC_X),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* 1070 */  (ISC_C,IMC_C), (ISC_VA,IMC_T), (ISC_VA,IMC_T), (ISC_VA,IMC_T), (ISC_VA,IMC_T),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* 1078 */  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* 1080 */  (ISC_C,IMC_C),  (ISC_C,IMC_C), (ISC_CM,IMC_C), (ISC_VR,IMC_R), (ISC_VL,IMC_L), (ISC_VA,IMC_T), (ISC_VA,IMC_T),  (ISC_N,IMC_X),
  /* 1088 */  (ISC_N,IMC_X),  (ISC_N,IMC_X),  (ISC_N,IMC_X),  (ISC_N,IMC_X),  (ISC_N,IMC_X),  (ISC_N,IMC_X),  (ISC_C,IMC_C),  (ISC_N,IMC_X),
  /* 1090 */ (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C),
  /* 1098 */ (ISC_GB,IMC_C), (ISC_GB,IMC_C),  (ISC_N,IMC_X),  (ISC_N,IMC_X), (ISC_VR,IMC_R), (ISC_VA,IMC_T),  (ISC_X,IMC_X),  (ISC_X,IMC_X),

  /* Khmer */

  /* 1780 */  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* 1788 */  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* 1790 */  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* 1798 */  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* 17A0 */  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),
  /* 17A8 */  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),
  /* 17B0 */  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_V,IMC_C),  (ISC_X,IMC_X),  (ISC_X,IMC_X), (ISC_VR,IMC_R), (ISC_VA,IMC_T),
  /* 17B8 */ (ISC_VA,IMC_T), (ISC_VA,IMC_T), (ISC_VA,IMC_T), (ISC_VB,IMC_B), (ISC_VB,IMC_B), (ISC_VB,IMC_B), (ISC_VA,IMC_T), (ISC_VR,IMC_R),
  /* 17C0 */ (ISC_VR,IMC_R), (ISC_VL,IMC_L), (ISC_VL,IMC_L), (ISC_VL,IMC_L), (ISC_VR,IMC_R), (ISC_VR,IMC_R),(ISC_SM,IMC_SM),(ISC_SM,IMC_SM),
  /* 17C8 */ (ISC_VR,IMC_R), (ISC_RS,IMC_T), (ISC_RS,IMC_T),(ISC_SM,IMC_SM), (ISC_CM,IMC_C), (ISC_VA,IMC_T),(ISC_SM,IMC_SM),(ISC_SM,IMC_SM),
  /* 17D0 */(ISC_SM,IMC_SM), (ISC_VA,IMC_T), (ISC_Co,IMC_X),(ISC_SM,IMC_SM),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),
  /* 17D8 */  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X), (ISC_S,IMC_SM),(ISC_SM,IMC_SM),  (ISC_X,IMC_X),  (ISC_X,IMC_X),
  /* 17E0 */ (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C),
  /* 17E8 */ (ISC_GB,IMC_C), (ISC_GB,IMC_C),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),

  /* Vedic Extensions */

  /* 1CD0 */ (ISC_A,IMC_SM), (ISC_A,IMC_SM), (ISC_A,IMC_SM),  (ISC_X,IMC_X), (ISC_A,IMC_SM), (ISC_A,IMC_SM), (ISC_A,IMC_SM), (ISC_A,IMC_SM),
  /* 1CD8 */ (ISC_A,IMC_SM), (ISC_A,IMC_SM), (ISC_A,IMC_SM), (ISC_A,IMC_SM), (ISC_A,IMC_SM), (ISC_A,IMC_SM), (ISC_A,IMC_SM), (ISC_A,IMC_SM),
  /* 1CE0 */ (ISC_A,IMC_SM), (ISC_A,IMC_SM), (ISC_A,IMC_SM), (ISC_A,IMC_SM), (ISC_A,IMC_SM), (ISC_A,IMC_SM), (ISC_A,IMC_SM), (ISC_A,IMC_SM),
  /* 1CE8 */ (ISC_A,IMC_SM), (ISC_S,IMC_SM), (ISC_S,IMC_SM), (ISC_S,IMC_SM), (ISC_S,IMC_SM), (ISC_A,IMC_SM), (ISC_S,IMC_SM), (ISC_S,IMC_SM),

  /* No_Block */

  /* 1CF0 */ (ISC_S,IMC_SM), (ISC_S,IMC_SM),  (ISC_C,IMC_C),  (ISC_C,IMC_C), (ISC_A,IMC_SM),  (ISC_C,IMC_C),  (ISC_C,IMC_C), (ISC_A,IMC_SM),

  /* Vedic Extensions */

  /* 1CF8 */ (ISC_A,IMC_SM), (ISC_A,IMC_SM), (ISC_GB,IMC_C),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),

  /* General Punctuation */

  /* 2008 */  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),(ISC_ZWNJ,IMC_X),(ISC_ZWJ,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),
  /* 2010 */ (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),

  /* Superscripts and Subscripts */

  /* 2070 */  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),(ISC_SM,IMC_SM),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),
  /* 2078 */  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),
  /* 2080 */  (ISC_X,IMC_X),  (ISC_X,IMC_X),(ISC_SM,IMC_SM),(ISC_SM,IMC_SM),(ISC_SM,IMC_SM),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),

  /* Devanagari Extended */

  /* A8E0 */ (ISC_A,IMC_SM), (ISC_A,IMC_SM), (ISC_A,IMC_SM), (ISC_A,IMC_SM), (ISC_A,IMC_SM), (ISC_A,IMC_SM), (ISC_A,IMC_SM), (ISC_A,IMC_SM),
  /* A8E8 */ (ISC_A,IMC_SM), (ISC_A,IMC_SM), (ISC_A,IMC_SM), (ISC_A,IMC_SM), (ISC_A,IMC_SM), (ISC_A,IMC_SM), (ISC_A,IMC_SM), (ISC_A,IMC_SM),
  /* A8F0 */ (ISC_A,IMC_SM), (ISC_A,IMC_SM), (ISC_S,IMC_SM), (ISC_S,IMC_SM), (ISC_S,IMC_SM), (ISC_S,IMC_SM), (ISC_S,IMC_SM), (ISC_S,IMC_SM),
  /* A8F8 */  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_V,IMC_C), (ISC_M,IMC_AS),

  /* Myanmar Extended-B */

  /* A9E0 */  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C), (ISC_VA,IMC_T),  (ISC_X,IMC_X),  (ISC_C,IMC_C),
  /* A9E8 */  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* A9F0 */ (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C),
  /* A9F8 */ (ISC_GB,IMC_C), (ISC_GB,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_X,IMC_X),

  /* Myanmar Extended-A */

  /* AA60 */  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* AA68 */  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C),
  /* AA70 */  (ISC_X,IMC_X),  (ISC_C,IMC_C),  (ISC_C,IMC_C),  (ISC_C,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C), (ISC_GB,IMC_C),  (ISC_X,IMC_X),
  /* AA78 */  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_C,IMC_C),  (ISC_N,IMC_X),  (ISC_N,IMC_X),  (ISC_N,IMC_X),  (ISC_C,IMC_C),  (ISC_C,IMC_C),

  /* No_Block */

  /* 11300 */  (ISC_X,IMC_X),(ISC_SM,IMC_SM),(ISC_SM,IMC_SM),(ISC_SM,IMC_SM),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),
  /* 11308 */  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),
  /* 11310 */  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),
  /* 11318 */  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),
  /* 11320 */  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),
  /* 11328 */  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),
  /* 11330 */  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),
  /* 11338 */  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_N,IMC_X),  (ISC_N,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),  (ISC_X,IMC_X),

];

const OFFSET_0X0028: usize = 0;
const OFFSET_0X00B0: usize = 24;
const OFFSET_0X0900: usize = 64;
const OFFSET_0X1000: usize = 1336;
const OFFSET_0X1780: usize = 1496;
const OFFSET_0X1CD0: usize = 1608;
const OFFSET_0X2008: usize = 1656;
const OFFSET_0X2070: usize = 1672;
const OFFSET_0XA8E0: usize = 1696;
const OFFSET_0XA9E0: usize = 1728;
const OFFSET_0XAA60: usize = 1760;
const OFFSET_0X11300: usize = 1792;

#[rustfmt::skip]
pub fn get_categories(u: u32) -> (SyllabicCategory, MatraCategory) {
    match u >> 12 {
        0x0 => {
            if u == 0x00A0 { return (ISC_GB, IMC_C); }
            if (0x0028..=0x003F).contains(&u) { return TABLE[u as usize - 0x0028 + OFFSET_0X0028]; }
            if (0x00B0..=0x00D7).contains(&u) { return TABLE[u as usize - 0x00B0 + OFFSET_0X00B0]; }
            if (0x0900..=0x0DF7).contains(&u) { return TABLE[u as usize - 0x0900 + OFFSET_0X0900]; }
        }
        0x1 => {
            if (0x1000..=0x109F).contains(&u) { return TABLE[u as usize - 0x1000 + OFFSET_0X1000]; }
            if (0x1780..=0x17EF).contains(&u) { return TABLE[u as usize - 0x1780 + OFFSET_0X1780]; }
            if (0x1CD0..=0x1CFF).contains(&u) { return TABLE[u as usize - 0x1CD0 + OFFSET_0X1CD0]; }
        }
        0x2 => {
            if u == 0x25CC { return (ISC_DC, IMC_C); }
            if (0x2008..=0x2017).contains(&u) { return TABLE[u as usize - 0x2008 + OFFSET_0X2008]; }
            if (0x2070..=0x2087).contains(&u) { return TABLE[u as usize - 0x2070 + OFFSET_0X2070]; }
        }
        0xA => {
            if (0xA8E0..=0xA8FF).contains(&u) { return TABLE[u as usize - 0xA8E0 + OFFSET_0XA8E0]; }
            if (0xA9E0..=0xA9FF).contains(&u) { return TABLE[u as usize - 0xA9E0 + OFFSET_0XA9E0]; }
            if (0xAA60..=0xAA7F).contains(&u) { return TABLE[u as usize - 0xAA60 + OFFSET_0XAA60]; }
        }
        0x11 => {
            if (0x11300..=0x1133F).contains(&u) { return TABLE[u as usize - 0x11300 + OFFSET_0X11300]; }
        }
        _ => {}
    }

    (ISC_X, IMC_X)
}
