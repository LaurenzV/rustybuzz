// WARNING: this file was generated by scripts/gen-indic-table.py

#![allow(non_camel_case_types)]
#![allow(unused_imports)]

use super::ot_shaper_indic::ot_category_t::*;
use super::ot_shaper_indic::ot_position_t::*;

use OT_A     as _OT_A;          /*  53 chars; A */
use OT_As    as _OT_As;         /*   1 chars; As */
use OT_C     as _OT_C;          /* 478 chars; C */
use OT_CM    as _OT_CM;         /*   1 chars; CM */
use OT_CS    as _OT_CS;         /*   2 chars; CS */
use OT_DOTTEDCIRCLE as _OT_DC;         /*   1 chars; DOTTEDCIRCLE */
use OT_H     as _OT_H;          /*  11 chars; H */
use OT_M     as _OT_M;          /* 142 chars; M */
use OT_MH    as _OT_MH;         /*   1 chars; MH */
use OT_ML    as _OT_ML;         /*   1 chars; ML */
use OT_MPst  as _OT_MP;         /*   1 chars; MPst */
use OT_MR    as _OT_MR;         /*   1 chars; MR */
use OT_MW    as _OT_MW;         /*   2 chars; MW */
use OT_MY    as _OT_MY;         /*   3 chars; MY */
use OT_N     as _OT_N;          /*  17 chars; N */
use OT_PLACEHOLDER as _OT_GB;         /* 164 chars; PLACEHOLDER */
use OT_PT    as _OT_PT;         /*   8 chars; PT */
use OT_Ra    as _OT_R;          /*  14 chars; Ra */
use OT_Repha as _OT_Rf;         /*   1 chars; Repha */
use OT_Robatic as _OT_Rt;         /*   3 chars; Robatic */
use OT_SM    as _OT_SM;         /*  56 chars; SM */
use OT_Symbol as _OT_S;          /*  22 chars; Symbol */
use OT_V     as _OT_V;          /* 172 chars; V */
use OT_VAbv  as _OT_VA;         /*  18 chars; VAbv */
use OT_VBlw  as _OT_VB;         /*   7 chars; VBlw */
use OT_VPre  as _OT_VL;         /*   5 chars; VPre */
use OT_VPst  as _OT_VR;         /*  13 chars; VPst */
use OT_VS    as _OT_VS;         /*  16 chars; VS */
use OT_X     as _OT_X;          /*   2 chars; X */
use OT_Xgroup as _OT_Xg;         /*   7 chars; Xgroup */
use OT_Ygroup as _OT_Yg;         /*   4 chars; Ygroup */
use OT_ZWJ   as _OT_ZWJ;        /*   1 chars; ZWJ */
use OT_ZWNJ  as _OT_ZWNJ;       /*   1 chars; ZWNJ */

use POS_ABOVE_C as _POS_T;         /*  22 chars; ABOVE_C */
use POS_AFTER_MAIN as _POS_A;         /*   3 chars; AFTER_MAIN */
use POS_AFTER_POST as _POS_AP;        /*  50 chars; AFTER_POST */
use POS_AFTER_SUB as _POS_AS;        /*  51 chars; AFTER_SUB */
use POS_BASE_C as _POS_C;         /* 832 chars; BASE_C */
use POS_BEFORE_SUB as _POS_BS;        /*  25 chars; BEFORE_SUB */
use POS_BELOW_C as _POS_B;         /*  13 chars; BELOW_C */
use POS_END  as _POS_X;         /*  71 chars; END */
use POS_POST_C as _POS_R;         /*  13 chars; POST_C */
use POS_PRE_C as _POS_L;         /*   5 chars; PRE_C */
use POS_PRE_M as _POS_LM;        /*  14 chars; PRE_M */
use POS_SMVD as _POS_SM;        /* 130 chars; SMVD */


pub type SyllabicCategory = u8;
pub type MatraCategory = u8;

#[rustfmt::skip]
const TABLE: &[(SyllabicCategory, MatraCategory)] = &[


  /* Basic Latin */

  /* 0028 */ (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X),(_OT_GB,_POS_C), (_OT_X,_POS_X), (_OT_X,_POS_X),
  /* 0030 */(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),
  /* 0038 */(_OT_GB,_POS_C),(_OT_GB,_POS_C), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X),

  /* Latin-1 Supplement */

  /* 00B0 */ (_OT_X,_POS_X), (_OT_X,_POS_X),(_OT_SM,_POS_SM),(_OT_SM,_POS_SM), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X),
  /* 00B8 */ (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X),
  /* 00C0 */ (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X),
  /* 00C8 */ (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X),
  /* 00D0 */ (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X),(_OT_GB,_POS_C),

  /* Devanagari */

  /* 0900 */(_OT_SM,_POS_SM),(_OT_SM,_POS_SM),(_OT_SM,_POS_SM),(_OT_SM,_POS_SM), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C),
  /* 0908 */ (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C),
  /* 0910 */ (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),
  /* 0918 */ (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),
  /* 0920 */ (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),
  /* 0928 */ (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),
  /* 0930 */ (_OT_R,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),
  /* 0938 */ (_OT_C,_POS_C), (_OT_C,_POS_C),(_OT_M,_POS_AS),(_OT_M,_POS_AS), (_OT_N,_POS_X),(_OT_S,_POS_SM),(_OT_M,_POS_AS),(_OT_M,_POS_LM),
  /* 0940 */(_OT_M,_POS_AS),(_OT_M,_POS_AS),(_OT_M,_POS_AS),(_OT_M,_POS_AS),(_OT_M,_POS_AS),(_OT_M,_POS_AS),(_OT_M,_POS_AS),(_OT_M,_POS_AS),
  /* 0948 */(_OT_M,_POS_AS),(_OT_M,_POS_AS),(_OT_M,_POS_AS),(_OT_M,_POS_AS),(_OT_M,_POS_AS), (_OT_H,_POS_B),(_OT_M,_POS_LM),(_OT_M,_POS_AS),
  /* 0950 */ (_OT_X,_POS_X),(_OT_A,_POS_SM),(_OT_A,_POS_SM),(_OT_SM,_POS_SM),(_OT_SM,_POS_SM),(_OT_M,_POS_AS),(_OT_M,_POS_AS),(_OT_M,_POS_AS),
  /* 0958 */ (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),
  /* 0960 */ (_OT_V,_POS_C), (_OT_V,_POS_C),(_OT_M,_POS_AS),(_OT_M,_POS_AS), (_OT_X,_POS_X), (_OT_X,_POS_X),(_OT_GB,_POS_C),(_OT_GB,_POS_C),
  /* 0968 */(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),
  /* 0970 */ (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C),
  /* 0978 */ (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),

  /* Bengali */

  /* 0980 */(_OT_GB,_POS_C),(_OT_SM,_POS_SM),(_OT_SM,_POS_SM),(_OT_SM,_POS_SM), (_OT_X,_POS_X), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C),
  /* 0988 */ (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_V,_POS_C),
  /* 0990 */ (_OT_V,_POS_C), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),
  /* 0998 */ (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),
  /* 09A0 */ (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),
  /* 09A8 */ (_OT_C,_POS_C), (_OT_X,_POS_X), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),
  /* 09B0 */ (_OT_R,_POS_C), (_OT_X,_POS_X), (_OT_C,_POS_C), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_C,_POS_C), (_OT_C,_POS_C),
  /* 09B8 */ (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_N,_POS_X),(_OT_S,_POS_SM),(_OT_M,_POS_AP),(_OT_M,_POS_LM),
  /* 09C0 */(_OT_M,_POS_AP),(_OT_M,_POS_AS),(_OT_M,_POS_AS),(_OT_M,_POS_AS),(_OT_M,_POS_AS), (_OT_X,_POS_X), (_OT_X,_POS_X),(_OT_M,_POS_LM),
  /* 09C8 */(_OT_M,_POS_LM), (_OT_X,_POS_X), (_OT_X,_POS_X),(_OT_M,_POS_AP),(_OT_M,_POS_AP), (_OT_H,_POS_B), (_OT_C,_POS_C), (_OT_X,_POS_X),
  /* 09D0 */ (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X),(_OT_M,_POS_AP),
  /* 09D8 */ (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_X,_POS_X), (_OT_C,_POS_C),
  /* 09E0 */ (_OT_V,_POS_C), (_OT_V,_POS_C),(_OT_M,_POS_AS),(_OT_M,_POS_AS), (_OT_X,_POS_X), (_OT_X,_POS_X),(_OT_GB,_POS_C),(_OT_GB,_POS_C),
  /* 09E8 */(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),
  /* 09F0 */ (_OT_R,_POS_C), (_OT_C,_POS_C), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X),
  /* 09F8 */ (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X),(_OT_GB,_POS_C), (_OT_X,_POS_X),(_OT_SM,_POS_SM), (_OT_X,_POS_X),

  /* Gurmukhi */

  /* 0A00 */ (_OT_X,_POS_X),(_OT_SM,_POS_SM),(_OT_SM,_POS_SM),(_OT_SM,_POS_SM), (_OT_X,_POS_X), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C),
  /* 0A08 */ (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_V,_POS_C),
  /* 0A10 */ (_OT_V,_POS_C), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),
  /* 0A18 */ (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),
  /* 0A20 */ (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),
  /* 0A28 */ (_OT_C,_POS_C), (_OT_X,_POS_X), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),
  /* 0A30 */ (_OT_R,_POS_C), (_OT_X,_POS_X), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_X,_POS_X), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_X,_POS_X),
  /* 0A38 */ (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_N,_POS_X), (_OT_X,_POS_X),(_OT_M,_POS_AP),(_OT_M,_POS_LM),
  /* 0A40 */(_OT_MP,_POS_AP),(_OT_M,_POS_AP),(_OT_M,_POS_AP), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X),(_OT_M,_POS_AP),
  /* 0A48 */(_OT_M,_POS_AP), (_OT_X,_POS_X), (_OT_X,_POS_X),(_OT_M,_POS_AP),(_OT_M,_POS_AP), (_OT_H,_POS_B), (_OT_X,_POS_X), (_OT_X,_POS_X),
  /* 0A50 */ (_OT_X,_POS_X), (_OT_M,_POS_B), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X),
  /* 0A58 */ (_OT_X,_POS_X), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_X,_POS_X), (_OT_C,_POS_C), (_OT_X,_POS_X),
  /* 0A60 */ (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X),(_OT_GB,_POS_C),(_OT_GB,_POS_C),
  /* 0A68 */(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),
  /* 0A70 */(_OT_SM,_POS_SM),(_OT_SM,_POS_SM), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_X,_POS_X),(_OT_CM,_POS_C), (_OT_X,_POS_X), (_OT_X,_POS_X),
  /* 0A78 */ (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X),

  /* Gujarati */

  /* 0A80 */ (_OT_X,_POS_X),(_OT_SM,_POS_SM),(_OT_SM,_POS_SM),(_OT_SM,_POS_SM), (_OT_X,_POS_X), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C),
  /* 0A88 */ (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_X,_POS_X), (_OT_V,_POS_C),
  /* 0A90 */ (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_X,_POS_X), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),
  /* 0A98 */ (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),
  /* 0AA0 */ (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),
  /* 0AA8 */ (_OT_C,_POS_C), (_OT_X,_POS_X), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),
  /* 0AB0 */ (_OT_R,_POS_C), (_OT_X,_POS_X), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_X,_POS_X), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),
  /* 0AB8 */ (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_N,_POS_X),(_OT_S,_POS_SM),(_OT_M,_POS_AP),(_OT_M,_POS_LM),
  /* 0AC0 */(_OT_M,_POS_AP),(_OT_M,_POS_AP),(_OT_M,_POS_AP),(_OT_M,_POS_AP),(_OT_M,_POS_AP),(_OT_M,_POS_AS), (_OT_X,_POS_X),(_OT_M,_POS_AS),
  /* 0AC8 */(_OT_M,_POS_AS),(_OT_M,_POS_AP), (_OT_X,_POS_X),(_OT_M,_POS_AP),(_OT_M,_POS_AP), (_OT_H,_POS_B), (_OT_X,_POS_X), (_OT_X,_POS_X),
  /* 0AD0 */ (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X),
  /* 0AD8 */ (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X),
  /* 0AE0 */ (_OT_V,_POS_C), (_OT_V,_POS_C),(_OT_M,_POS_AP),(_OT_M,_POS_AP), (_OT_X,_POS_X), (_OT_X,_POS_X),(_OT_GB,_POS_C),(_OT_GB,_POS_C),
  /* 0AE8 */(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),
  /* 0AF0 */ (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X),
  /* 0AF8 */ (_OT_X,_POS_X), (_OT_C,_POS_C),(_OT_A,_POS_SM), (_OT_N,_POS_X),(_OT_A,_POS_SM), (_OT_N,_POS_X), (_OT_N,_POS_X), (_OT_N,_POS_X),

  /* Oriya */

  /* 0B00 */ (_OT_X,_POS_X),(_OT_SM,_POS_BS),(_OT_SM,_POS_SM),(_OT_SM,_POS_SM), (_OT_X,_POS_X), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C),
  /* 0B08 */ (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_V,_POS_C),
  /* 0B10 */ (_OT_V,_POS_C), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),
  /* 0B18 */ (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),
  /* 0B20 */ (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),
  /* 0B28 */ (_OT_C,_POS_C), (_OT_X,_POS_X), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),
  /* 0B30 */ (_OT_R,_POS_C), (_OT_X,_POS_X), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_X,_POS_X), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),
  /* 0B38 */ (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_N,_POS_X),(_OT_S,_POS_SM),(_OT_M,_POS_AP), (_OT_M,_POS_A),
  /* 0B40 */(_OT_M,_POS_AP),(_OT_M,_POS_AS),(_OT_M,_POS_AS),(_OT_M,_POS_AS),(_OT_M,_POS_AS), (_OT_X,_POS_X), (_OT_X,_POS_X),(_OT_M,_POS_LM),
  /* 0B48 */ (_OT_M,_POS_A), (_OT_X,_POS_X), (_OT_X,_POS_X),(_OT_M,_POS_AP),(_OT_M,_POS_AP), (_OT_H,_POS_B), (_OT_X,_POS_X), (_OT_X,_POS_X),
  /* 0B50 */ (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_N,_POS_X), (_OT_M,_POS_A),(_OT_M,_POS_AP),
  /* 0B58 */ (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_X,_POS_X), (_OT_C,_POS_C),
  /* 0B60 */ (_OT_V,_POS_C), (_OT_V,_POS_C),(_OT_M,_POS_AS),(_OT_M,_POS_AS), (_OT_X,_POS_X), (_OT_X,_POS_X),(_OT_GB,_POS_C),(_OT_GB,_POS_C),
  /* 0B68 */(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),
  /* 0B70 */ (_OT_X,_POS_X), (_OT_C,_POS_C), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X),
  /* 0B78 */ (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X),

  /* Tamil */

  /* 0B80 */ (_OT_X,_POS_X), (_OT_X,_POS_X),(_OT_SM,_POS_SM), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C),
  /* 0B88 */ (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_V,_POS_C), (_OT_V,_POS_C),
  /* 0B90 */ (_OT_V,_POS_C), (_OT_X,_POS_X), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_C,_POS_C), (_OT_X,_POS_X), (_OT_X,_POS_X),
  /* 0B98 */ (_OT_X,_POS_X), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_X,_POS_X), (_OT_C,_POS_C), (_OT_X,_POS_X), (_OT_C,_POS_C), (_OT_C,_POS_C),
  /* 0BA0 */ (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X),
  /* 0BA8 */ (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_C,_POS_C), (_OT_C,_POS_C),
  /* 0BB0 */ (_OT_R,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),
  /* 0BB8 */ (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X),(_OT_M,_POS_AP),(_OT_M,_POS_AP),
  /* 0BC0 */(_OT_M,_POS_AS),(_OT_M,_POS_AP),(_OT_M,_POS_AP), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X),(_OT_M,_POS_LM),(_OT_M,_POS_LM),
  /* 0BC8 */(_OT_M,_POS_LM), (_OT_X,_POS_X),(_OT_M,_POS_AP),(_OT_M,_POS_AP),(_OT_M,_POS_AP), (_OT_H,_POS_T), (_OT_X,_POS_X), (_OT_X,_POS_X),
  /* 0BD0 */ (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X),(_OT_M,_POS_AP),
  /* 0BD8 */ (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X),
  /* 0BE0 */ (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X),(_OT_GB,_POS_C),(_OT_GB,_POS_C),
  /* 0BE8 */(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),
  /* 0BF0 */ (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X),
  /* 0BF8 */ (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X),

  /* Telugu */

  /* 0C00 */(_OT_SM,_POS_SM),(_OT_SM,_POS_SM),(_OT_SM,_POS_SM),(_OT_SM,_POS_SM),(_OT_SM,_POS_SM), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C),
  /* 0C08 */ (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_X,_POS_X), (_OT_V,_POS_C), (_OT_V,_POS_C),
  /* 0C10 */ (_OT_V,_POS_C), (_OT_X,_POS_X), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),
  /* 0C18 */ (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),
  /* 0C20 */ (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),
  /* 0C28 */ (_OT_C,_POS_C), (_OT_X,_POS_X), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),
  /* 0C30 */ (_OT_R,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),
  /* 0C38 */ (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_N,_POS_X),(_OT_S,_POS_SM),(_OT_M,_POS_BS),(_OT_M,_POS_BS),
  /* 0C40 */(_OT_M,_POS_BS),(_OT_M,_POS_BS),(_OT_M,_POS_BS),(_OT_M,_POS_AS),(_OT_M,_POS_AS), (_OT_X,_POS_X),(_OT_M,_POS_BS),(_OT_M,_POS_BS),
  /* 0C48 */(_OT_M,_POS_BS), (_OT_X,_POS_X),(_OT_M,_POS_BS),(_OT_M,_POS_BS),(_OT_M,_POS_BS), (_OT_H,_POS_T), (_OT_X,_POS_X), (_OT_X,_POS_X),
  /* 0C50 */ (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X),(_OT_M,_POS_BS),(_OT_M,_POS_BS), (_OT_X,_POS_X),
  /* 0C58 */ (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_C,_POS_C), (_OT_X,_POS_X), (_OT_X,_POS_X),
  /* 0C60 */ (_OT_V,_POS_C), (_OT_V,_POS_C),(_OT_M,_POS_BS),(_OT_M,_POS_BS), (_OT_X,_POS_X), (_OT_X,_POS_X),(_OT_GB,_POS_C),(_OT_GB,_POS_C),
  /* 0C68 */(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),
  /* 0C70 */ (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X),
  /* 0C78 */ (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X),

  /* Kannada */

  /* 0C80 */(_OT_GB,_POS_C),(_OT_SM,_POS_SM),(_OT_SM,_POS_SM),(_OT_SM,_POS_SM), (_OT_X,_POS_X), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C),
  /* 0C88 */ (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_X,_POS_X), (_OT_V,_POS_C), (_OT_V,_POS_C),
  /* 0C90 */ (_OT_V,_POS_C), (_OT_X,_POS_X), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),
  /* 0C98 */ (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),
  /* 0CA0 */ (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),
  /* 0CA8 */ (_OT_C,_POS_C), (_OT_X,_POS_X), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),
  /* 0CB0 */ (_OT_R,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_X,_POS_X), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),
  /* 0CB8 */ (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_N,_POS_X),(_OT_S,_POS_SM),(_OT_M,_POS_BS),(_OT_M,_POS_BS),
  /* 0CC0 */(_OT_M,_POS_BS),(_OT_M,_POS_BS),(_OT_M,_POS_BS),(_OT_M,_POS_AS),(_OT_M,_POS_AS), (_OT_X,_POS_X),(_OT_M,_POS_BS),(_OT_M,_POS_AS),
  /* 0CC8 */(_OT_M,_POS_AS), (_OT_X,_POS_X),(_OT_M,_POS_AS),(_OT_M,_POS_AS),(_OT_M,_POS_BS), (_OT_H,_POS_T), (_OT_X,_POS_X), (_OT_X,_POS_X),
  /* 0CD0 */ (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X),(_OT_M,_POS_AS),(_OT_M,_POS_AS), (_OT_X,_POS_X),
  /* 0CD8 */ (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_X,_POS_X),
  /* 0CE0 */ (_OT_V,_POS_C), (_OT_V,_POS_C),(_OT_M,_POS_BS),(_OT_M,_POS_BS), (_OT_X,_POS_X), (_OT_X,_POS_X),(_OT_GB,_POS_C),(_OT_GB,_POS_C),
  /* 0CE8 */(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),
  /* 0CF0 */ (_OT_X,_POS_X),(_OT_CS,_POS_C),(_OT_CS,_POS_C),(_OT_SM,_POS_SM), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X),
  /* 0CF8 */ (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X),

  /* Malayalam */

  /* 0D00 */(_OT_SM,_POS_SM),(_OT_SM,_POS_SM),(_OT_SM,_POS_SM),(_OT_SM,_POS_SM),(_OT_GB,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C),
  /* 0D08 */ (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_X,_POS_X), (_OT_V,_POS_C), (_OT_V,_POS_C),
  /* 0D10 */ (_OT_V,_POS_C), (_OT_X,_POS_X), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),
  /* 0D18 */ (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),
  /* 0D20 */ (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),
  /* 0D28 */ (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),
  /* 0D30 */ (_OT_R,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),
  /* 0D38 */ (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),(_OT_M,_POS_AS),(_OT_M,_POS_AS),(_OT_S,_POS_SM),(_OT_M,_POS_AP),(_OT_M,_POS_AP),
  /* 0D40 */(_OT_M,_POS_AP),(_OT_M,_POS_AP),(_OT_M,_POS_AP),(_OT_M,_POS_AP),(_OT_M,_POS_AP), (_OT_X,_POS_X),(_OT_M,_POS_LM),(_OT_M,_POS_LM),
  /* 0D48 */(_OT_M,_POS_LM), (_OT_X,_POS_X),(_OT_M,_POS_AP),(_OT_M,_POS_AP),(_OT_M,_POS_AP), (_OT_H,_POS_T),(_OT_Rf,_POS_X), (_OT_X,_POS_X),
  /* 0D50 */ (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),(_OT_M,_POS_AP),
  /* 0D58 */ (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_V,_POS_C),
  /* 0D60 */ (_OT_V,_POS_C), (_OT_V,_POS_C),(_OT_M,_POS_AP),(_OT_M,_POS_AP), (_OT_X,_POS_X), (_OT_X,_POS_X),(_OT_GB,_POS_C),(_OT_GB,_POS_C),
  /* 0D68 */(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),
  /* 0D70 */ (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X),
  /* 0D78 */ (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),

  /* Myanmar */

  /* 1000 */ (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_R,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),
  /* 1008 */ (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),
  /* 1010 */ (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),
  /* 1018 */ (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_R,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),
  /* 1020 */ (_OT_C,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C),
  /* 1028 */ (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C),(_OT_VR,_POS_R),(_OT_VR,_POS_R),(_OT_VA,_POS_T),(_OT_VA,_POS_T),(_OT_VB,_POS_B),
  /* 1030 */(_OT_VB,_POS_B),(_OT_VL,_POS_L),(_OT_A,_POS_SM),(_OT_VA,_POS_T),(_OT_VA,_POS_T),(_OT_VA,_POS_T),(_OT_A,_POS_SM), (_OT_N,_POS_X),
  /* 1038 */(_OT_SM,_POS_SM), (_OT_H,_POS_X),(_OT_As,_POS_X),(_OT_MY,_POS_X),(_OT_MR,_POS_X),(_OT_MW,_POS_X),(_OT_MH,_POS_X), (_OT_C,_POS_C),
  /* 1040 */(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),
  /* 1048 */(_OT_GB,_POS_C),(_OT_GB,_POS_C), (_OT_X,_POS_X),(_OT_GB,_POS_C), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_C,_POS_C), (_OT_X,_POS_X),
  /* 1050 */ (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C),(_OT_VR,_POS_R),(_OT_VR,_POS_R),
  /* 1058 */(_OT_VB,_POS_B),(_OT_VB,_POS_B), (_OT_R,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),(_OT_MY,_POS_X),(_OT_MY,_POS_X),
  /* 1060 */(_OT_ML,_POS_X), (_OT_C,_POS_C),(_OT_VR,_POS_R),(_OT_PT,_POS_X),(_OT_PT,_POS_X), (_OT_C,_POS_C), (_OT_C,_POS_C),(_OT_VR,_POS_R),
  /* 1068 */(_OT_VR,_POS_R),(_OT_PT,_POS_X),(_OT_PT,_POS_X),(_OT_PT,_POS_X),(_OT_PT,_POS_X),(_OT_PT,_POS_X), (_OT_C,_POS_C), (_OT_C,_POS_C),
  /* 1070 */ (_OT_C,_POS_C),(_OT_VA,_POS_T),(_OT_VA,_POS_T),(_OT_VA,_POS_T),(_OT_VA,_POS_T), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),
  /* 1078 */ (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),
  /* 1080 */ (_OT_C,_POS_C), (_OT_C,_POS_C),(_OT_MW,_POS_X),(_OT_VR,_POS_R),(_OT_VL,_POS_L),(_OT_VA,_POS_T),(_OT_VA,_POS_T),(_OT_SM,_POS_SM),
  /* 1088 */(_OT_SM,_POS_SM),(_OT_SM,_POS_SM),(_OT_SM,_POS_SM),(_OT_SM,_POS_SM),(_OT_SM,_POS_SM),(_OT_SM,_POS_SM), (_OT_C,_POS_C),(_OT_SM,_POS_SM),
  /* 1090 */(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),
  /* 1098 */(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_SM,_POS_SM),(_OT_SM,_POS_SM),(_OT_SM,_POS_SM),(_OT_VA,_POS_T), (_OT_X,_POS_X), (_OT_X,_POS_X),

  /* Khmer */

  /* 1780 */ (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),
  /* 1788 */ (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),
  /* 1790 */ (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),
  /* 1798 */ (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_R,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),
  /* 17A0 */ (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C),
  /* 17A8 */ (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C),
  /* 17B0 */ (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_V,_POS_C), (_OT_X,_POS_X), (_OT_X,_POS_X),(_OT_VR,_POS_R),(_OT_VA,_POS_T),
  /* 17B8 */(_OT_VA,_POS_T),(_OT_VA,_POS_T),(_OT_VA,_POS_T),(_OT_VB,_POS_B),(_OT_VB,_POS_B),(_OT_VB,_POS_B),(_OT_VA,_POS_T),(_OT_VR,_POS_R),
  /* 17C0 */(_OT_VR,_POS_R),(_OT_VL,_POS_L),(_OT_VL,_POS_L),(_OT_VL,_POS_L),(_OT_VR,_POS_R),(_OT_VR,_POS_R),(_OT_Xg,_POS_X),(_OT_Yg,_POS_X),
  /* 17C8 */(_OT_Yg,_POS_X),(_OT_Rt,_POS_X),(_OT_Rt,_POS_X),(_OT_Xg,_POS_X),(_OT_Rt,_POS_X),(_OT_Xg,_POS_X),(_OT_Xg,_POS_X),(_OT_Xg,_POS_X),
  /* 17D0 */(_OT_Xg,_POS_X),(_OT_Xg,_POS_X), (_OT_H,_POS_X),(_OT_Yg,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X),
  /* 17D8 */ (_OT_X,_POS_X),(_OT_GB,_POS_C), (_OT_X,_POS_X), (_OT_X,_POS_X),(_OT_S,_POS_SM),(_OT_Yg,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X),
  /* 17E0 */(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),
  /* 17E8 */(_OT_GB,_POS_C),(_OT_GB,_POS_C), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X),

  /* Vedic Extensions */

  /* 1CD0 */(_OT_A,_POS_SM),(_OT_A,_POS_SM),(_OT_A,_POS_SM), (_OT_X,_POS_X),(_OT_A,_POS_SM),(_OT_A,_POS_SM),(_OT_A,_POS_SM),(_OT_A,_POS_SM),
  /* 1CD8 */(_OT_A,_POS_SM),(_OT_A,_POS_SM),(_OT_A,_POS_SM),(_OT_A,_POS_SM),(_OT_A,_POS_SM),(_OT_A,_POS_SM),(_OT_A,_POS_SM),(_OT_A,_POS_SM),
  /* 1CE0 */(_OT_A,_POS_SM),(_OT_A,_POS_SM),(_OT_A,_POS_SM),(_OT_A,_POS_SM),(_OT_A,_POS_SM),(_OT_A,_POS_SM),(_OT_A,_POS_SM),(_OT_A,_POS_SM),
  /* 1CE8 */(_OT_A,_POS_SM),(_OT_S,_POS_SM),(_OT_S,_POS_SM),(_OT_S,_POS_SM),(_OT_S,_POS_SM),(_OT_A,_POS_SM),(_OT_S,_POS_SM),(_OT_S,_POS_SM),
  /* 1CF0 */(_OT_S,_POS_SM),(_OT_S,_POS_SM), (_OT_C,_POS_C), (_OT_C,_POS_C),(_OT_A,_POS_SM), (_OT_C,_POS_C), (_OT_C,_POS_C),(_OT_A,_POS_SM),
  /* 1CF8 */(_OT_A,_POS_SM),(_OT_A,_POS_SM),(_OT_GB,_POS_C), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X),

  /* General Punctuation */

  /* 2008 */ (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X),(_OT_ZWNJ,_POS_X),(_OT_ZWJ,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X),
  /* 2010 */(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C), (_OT_X,_POS_X), (_OT_X,_POS_X),
  /* 2018 */ (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X),
  /* 2020 */ (_OT_X,_POS_X), (_OT_X,_POS_X),(_OT_GB,_POS_C), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X),

  /* Superscripts and Subscripts */

  /* 2070 */ (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X),(_OT_SM,_POS_SM), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X),
  /* 2078 */ (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X),
  /* 2080 */ (_OT_X,_POS_X), (_OT_X,_POS_X),(_OT_SM,_POS_SM),(_OT_SM,_POS_SM),(_OT_SM,_POS_SM), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X),

  /* Geometric Shapes */

  /* 25F8 */ (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C), (_OT_X,_POS_X),

  /* Devanagari Extended */

  /* A8E0 */(_OT_A,_POS_SM),(_OT_A,_POS_SM),(_OT_A,_POS_SM),(_OT_A,_POS_SM),(_OT_A,_POS_SM),(_OT_A,_POS_SM),(_OT_A,_POS_SM),(_OT_A,_POS_SM),
  /* A8E8 */(_OT_A,_POS_SM),(_OT_A,_POS_SM),(_OT_A,_POS_SM),(_OT_A,_POS_SM),(_OT_A,_POS_SM),(_OT_A,_POS_SM),(_OT_A,_POS_SM),(_OT_A,_POS_SM),
  /* A8F0 */(_OT_A,_POS_SM),(_OT_A,_POS_SM),(_OT_S,_POS_SM),(_OT_S,_POS_SM),(_OT_S,_POS_SM),(_OT_S,_POS_SM),(_OT_S,_POS_SM),(_OT_S,_POS_SM),
  /* A8F8 */ (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_V,_POS_C),(_OT_M,_POS_AS),

  /* Myanmar Extended-B */

  /* A9E0 */ (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),(_OT_VA,_POS_T), (_OT_X,_POS_X), (_OT_C,_POS_C),
  /* A9E8 */ (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),
  /* A9F0 */(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),
  /* A9F8 */(_OT_GB,_POS_C),(_OT_GB,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_X,_POS_X),

  /* Myanmar Extended-A */

  /* AA60 */ (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),
  /* AA68 */ (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),
  /* AA70 */ (_OT_X,_POS_X), (_OT_C,_POS_C), (_OT_C,_POS_C), (_OT_C,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C),(_OT_GB,_POS_C), (_OT_X,_POS_X),
  /* AA78 */ (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_C,_POS_C),(_OT_PT,_POS_X), (_OT_N,_POS_X), (_OT_N,_POS_X), (_OT_C,_POS_C), (_OT_C,_POS_C),

  /* Variation Selectors */

  /* FE00 */(_OT_VS,_POS_X),(_OT_VS,_POS_X),(_OT_VS,_POS_X),(_OT_VS,_POS_X),(_OT_VS,_POS_X),(_OT_VS,_POS_X),(_OT_VS,_POS_X),(_OT_VS,_POS_X),
  /* FE08 */(_OT_VS,_POS_X),(_OT_VS,_POS_X),(_OT_VS,_POS_X),(_OT_VS,_POS_X),(_OT_VS,_POS_X),(_OT_VS,_POS_X),(_OT_VS,_POS_X),(_OT_VS,_POS_X),

  /* Grantha */

  /* 11300 */ (_OT_X,_POS_X),(_OT_SM,_POS_SM),(_OT_SM,_POS_SM),(_OT_SM,_POS_SM), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X),
  /* 11338 */ (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_N,_POS_X), (_OT_N,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X), (_OT_X,_POS_X),

];

const OFFSET_0X0028: usize = 0;
const OFFSET_0X00B0: usize = 24;
const OFFSET_0X0900: usize = 64;
const OFFSET_0X1000: usize = 1216;
const OFFSET_0X1780: usize = 1376;
const OFFSET_0X1CD0: usize = 1488;
const OFFSET_0X2008: usize = 1536;
const OFFSET_0X2070: usize = 1568;
const OFFSET_0X25F8: usize = 1592;
const OFFSET_0XA8E0: usize = 1600;
const OFFSET_0XA9E0: usize = 1632;
const OFFSET_0XAA60: usize = 1664;
const OFFSET_0XFE00: usize = 1696;
const OFFSET_0X11300: usize = 1712;
const OFFSET_0X11338: usize = 1720;

#[rustfmt::skip]
pub fn get_categories(u: u32) -> (SyllabicCategory, MatraCategory) {
    match u >> 12 {
        0x0 => {
            if u == 0x00A0 { return (_OT_GB, _POS_C); }
            if (0x0028..=0x003F).contains(&u) { return TABLE[u as usize - 0x0028 + OFFSET_0X0028]; }
            if (0x00B0..=0x00D7).contains(&u) { return TABLE[u as usize - 0x00B0 + OFFSET_0X00B0]; }
            if (0x0900..=0x0D7F).contains(&u) { return TABLE[u as usize - 0x0900 + OFFSET_0X0900]; }
        }
        0x1 => {
            if (0x1000..=0x109F).contains(&u) { return TABLE[u as usize - 0x1000 + OFFSET_0X1000]; }
            if (0x1780..=0x17EF).contains(&u) { return TABLE[u as usize - 0x1780 + OFFSET_0X1780]; }
            if (0x1CD0..=0x1CFF).contains(&u) { return TABLE[u as usize - 0x1CD0 + OFFSET_0X1CD0]; }
        }
        0x2 => {
            if u == 0x25CC { return (_OT_DC, _POS_C); }
            if (0x2008..=0x2027).contains(&u) { return TABLE[u as usize - 0x2008 + OFFSET_0X2008]; }
            if (0x2070..=0x2087).contains(&u) { return TABLE[u as usize - 0x2070 + OFFSET_0X2070]; }
            if (0x25F8..=0x25FF).contains(&u) { return TABLE[u as usize - 0x25F8 + OFFSET_0X25F8]; }
        }
        0xA => {
            if (0xA8E0..=0xA8FF).contains(&u) { return TABLE[u as usize - 0xA8E0 + OFFSET_0XA8E0]; }
            if (0xA9E0..=0xA9FF).contains(&u) { return TABLE[u as usize - 0xA9E0 + OFFSET_0XA9E0]; }
            if (0xAA60..=0xAA7F).contains(&u) { return TABLE[u as usize - 0xAA60 + OFFSET_0XAA60]; }
        }
        0xF => {
            if (0xFE00..=0xFE0F).contains(&u) { return TABLE[u as usize - 0xFE00 + OFFSET_0XFE00]; }
        }
        0x11 => {
            if (0x11300..=0x11307).contains(&u) { return TABLE[u as usize - 0x11300 + OFFSET_0X11300]; }
            if (0x11338..=0x1133F).contains(&u) { return TABLE[u as usize - 0x11338 + OFFSET_0X11338]; }
        }
        _ => {}
    }

    (_OT_X, _POS_X)
}