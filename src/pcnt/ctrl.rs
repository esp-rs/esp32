#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PCNT_CLK_EN`"]
pub type PCNT_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCNT_CLK_EN`"]
pub struct PCNT_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT_CLK_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `PCNT_CNT_PAUSE_U7`"]
pub type PCNT_CNT_PAUSE_U7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCNT_CNT_PAUSE_U7`"]
pub struct PCNT_CNT_PAUSE_U7_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT_CNT_PAUSE_U7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `PCNT_PLUS_CNT_RST_U7`"]
pub type PCNT_PLUS_CNT_RST_U7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCNT_PLUS_CNT_RST_U7`"]
pub struct PCNT_PLUS_CNT_RST_U7_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT_PLUS_CNT_RST_U7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `PCNT_CNT_PAUSE_U6`"]
pub type PCNT_CNT_PAUSE_U6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCNT_CNT_PAUSE_U6`"]
pub struct PCNT_CNT_PAUSE_U6_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT_CNT_PAUSE_U6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `PCNT_PLUS_CNT_RST_U6`"]
pub type PCNT_PLUS_CNT_RST_U6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCNT_PLUS_CNT_RST_U6`"]
pub struct PCNT_PLUS_CNT_RST_U6_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT_PLUS_CNT_RST_U6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `PCNT_CNT_PAUSE_U5`"]
pub type PCNT_CNT_PAUSE_U5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCNT_CNT_PAUSE_U5`"]
pub struct PCNT_CNT_PAUSE_U5_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT_CNT_PAUSE_U5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `PCNT_PLUS_CNT_RST_U5`"]
pub type PCNT_PLUS_CNT_RST_U5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCNT_PLUS_CNT_RST_U5`"]
pub struct PCNT_PLUS_CNT_RST_U5_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT_PLUS_CNT_RST_U5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `PCNT_CNT_PAUSE_U4`"]
pub type PCNT_CNT_PAUSE_U4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCNT_CNT_PAUSE_U4`"]
pub struct PCNT_CNT_PAUSE_U4_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT_CNT_PAUSE_U4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `PCNT_PLUS_CNT_RST_U4`"]
pub type PCNT_PLUS_CNT_RST_U4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCNT_PLUS_CNT_RST_U4`"]
pub struct PCNT_PLUS_CNT_RST_U4_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT_PLUS_CNT_RST_U4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `PCNT_CNT_PAUSE_U3`"]
pub type PCNT_CNT_PAUSE_U3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCNT_CNT_PAUSE_U3`"]
pub struct PCNT_CNT_PAUSE_U3_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT_CNT_PAUSE_U3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `PCNT_PLUS_CNT_RST_U3`"]
pub type PCNT_PLUS_CNT_RST_U3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCNT_PLUS_CNT_RST_U3`"]
pub struct PCNT_PLUS_CNT_RST_U3_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT_PLUS_CNT_RST_U3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `PCNT_CNT_PAUSE_U2`"]
pub type PCNT_CNT_PAUSE_U2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCNT_CNT_PAUSE_U2`"]
pub struct PCNT_CNT_PAUSE_U2_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT_CNT_PAUSE_U2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `PCNT_PLUS_CNT_RST_U2`"]
pub type PCNT_PLUS_CNT_RST_U2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCNT_PLUS_CNT_RST_U2`"]
pub struct PCNT_PLUS_CNT_RST_U2_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT_PLUS_CNT_RST_U2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `PCNT_CNT_PAUSE_U1`"]
pub type PCNT_CNT_PAUSE_U1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCNT_CNT_PAUSE_U1`"]
pub struct PCNT_CNT_PAUSE_U1_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT_CNT_PAUSE_U1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `PCNT_PLUS_CNT_RST_U1`"]
pub type PCNT_PLUS_CNT_RST_U1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCNT_PLUS_CNT_RST_U1`"]
pub struct PCNT_PLUS_CNT_RST_U1_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT_PLUS_CNT_RST_U1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `PCNT_CNT_PAUSE_U0`"]
pub type PCNT_CNT_PAUSE_U0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCNT_CNT_PAUSE_U0`"]
pub struct PCNT_CNT_PAUSE_U0_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT_CNT_PAUSE_U0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `PCNT_PLUS_CNT_RST_U0`"]
pub type PCNT_PLUS_CNT_RST_U0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCNT_PLUS_CNT_RST_U0`"]
pub struct PCNT_PLUS_CNT_RST_U0_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT_PLUS_CNT_RST_U0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pcnt_clk_en(&self) -> PCNT_CLK_EN_R {
        PCNT_CLK_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Set this bit to pause unit7's counter."]
    #[inline(always)]
    pub fn pcnt_cnt_pause_u7(&self) -> PCNT_CNT_PAUSE_U7_R {
        PCNT_CNT_PAUSE_U7_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Set this bit to clear unit7's counter."]
    #[inline(always)]
    pub fn pcnt_plus_cnt_rst_u7(&self) -> PCNT_PLUS_CNT_RST_U7_R {
        PCNT_PLUS_CNT_RST_U7_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Set this bit to pause unit6's counter."]
    #[inline(always)]
    pub fn pcnt_cnt_pause_u6(&self) -> PCNT_CNT_PAUSE_U6_R {
        PCNT_CNT_PAUSE_U6_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Set this bit to clear unit6's counter."]
    #[inline(always)]
    pub fn pcnt_plus_cnt_rst_u6(&self) -> PCNT_PLUS_CNT_RST_U6_R {
        PCNT_PLUS_CNT_RST_U6_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Set this bit to pause unit5's counter."]
    #[inline(always)]
    pub fn pcnt_cnt_pause_u5(&self) -> PCNT_CNT_PAUSE_U5_R {
        PCNT_CNT_PAUSE_U5_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Set this bit to clear unit5's counter."]
    #[inline(always)]
    pub fn pcnt_plus_cnt_rst_u5(&self) -> PCNT_PLUS_CNT_RST_U5_R {
        PCNT_PLUS_CNT_RST_U5_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Set this bit to pause unit4's counter."]
    #[inline(always)]
    pub fn pcnt_cnt_pause_u4(&self) -> PCNT_CNT_PAUSE_U4_R {
        PCNT_CNT_PAUSE_U4_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Set this bit to clear unit4's counter."]
    #[inline(always)]
    pub fn pcnt_plus_cnt_rst_u4(&self) -> PCNT_PLUS_CNT_RST_U4_R {
        PCNT_PLUS_CNT_RST_U4_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Set this bit to pause unit3's counter."]
    #[inline(always)]
    pub fn pcnt_cnt_pause_u3(&self) -> PCNT_CNT_PAUSE_U3_R {
        PCNT_CNT_PAUSE_U3_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Set this bit to clear unit3's counter."]
    #[inline(always)]
    pub fn pcnt_plus_cnt_rst_u3(&self) -> PCNT_PLUS_CNT_RST_U3_R {
        PCNT_PLUS_CNT_RST_U3_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Set this bit to pause unit2's counter."]
    #[inline(always)]
    pub fn pcnt_cnt_pause_u2(&self) -> PCNT_CNT_PAUSE_U2_R {
        PCNT_CNT_PAUSE_U2_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Set this bit to clear unit2's counter."]
    #[inline(always)]
    pub fn pcnt_plus_cnt_rst_u2(&self) -> PCNT_PLUS_CNT_RST_U2_R {
        PCNT_PLUS_CNT_RST_U2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Set this bit to pause unit1's counter."]
    #[inline(always)]
    pub fn pcnt_cnt_pause_u1(&self) -> PCNT_CNT_PAUSE_U1_R {
        PCNT_CNT_PAUSE_U1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Set this bit to clear unit1's counter."]
    #[inline(always)]
    pub fn pcnt_plus_cnt_rst_u1(&self) -> PCNT_PLUS_CNT_RST_U1_R {
        PCNT_PLUS_CNT_RST_U1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Set this bit to pause unit0's counter."]
    #[inline(always)]
    pub fn pcnt_cnt_pause_u0(&self) -> PCNT_CNT_PAUSE_U0_R {
        PCNT_CNT_PAUSE_U0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Set this bit to clear unit0's counter."]
    #[inline(always)]
    pub fn pcnt_plus_cnt_rst_u0(&self) -> PCNT_PLUS_CNT_RST_U0_R {
        PCNT_PLUS_CNT_RST_U0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pcnt_clk_en(&mut self) -> PCNT_CLK_EN_W {
        PCNT_CLK_EN_W { w: self }
    }
    #[doc = "Bit 15 - Set this bit to pause unit7's counter."]
    #[inline(always)]
    pub fn pcnt_cnt_pause_u7(&mut self) -> PCNT_CNT_PAUSE_U7_W {
        PCNT_CNT_PAUSE_U7_W { w: self }
    }
    #[doc = "Bit 14 - Set this bit to clear unit7's counter."]
    #[inline(always)]
    pub fn pcnt_plus_cnt_rst_u7(&mut self) -> PCNT_PLUS_CNT_RST_U7_W {
        PCNT_PLUS_CNT_RST_U7_W { w: self }
    }
    #[doc = "Bit 13 - Set this bit to pause unit6's counter."]
    #[inline(always)]
    pub fn pcnt_cnt_pause_u6(&mut self) -> PCNT_CNT_PAUSE_U6_W {
        PCNT_CNT_PAUSE_U6_W { w: self }
    }
    #[doc = "Bit 12 - Set this bit to clear unit6's counter."]
    #[inline(always)]
    pub fn pcnt_plus_cnt_rst_u6(&mut self) -> PCNT_PLUS_CNT_RST_U6_W {
        PCNT_PLUS_CNT_RST_U6_W { w: self }
    }
    #[doc = "Bit 11 - Set this bit to pause unit5's counter."]
    #[inline(always)]
    pub fn pcnt_cnt_pause_u5(&mut self) -> PCNT_CNT_PAUSE_U5_W {
        PCNT_CNT_PAUSE_U5_W { w: self }
    }
    #[doc = "Bit 10 - Set this bit to clear unit5's counter."]
    #[inline(always)]
    pub fn pcnt_plus_cnt_rst_u5(&mut self) -> PCNT_PLUS_CNT_RST_U5_W {
        PCNT_PLUS_CNT_RST_U5_W { w: self }
    }
    #[doc = "Bit 9 - Set this bit to pause unit4's counter."]
    #[inline(always)]
    pub fn pcnt_cnt_pause_u4(&mut self) -> PCNT_CNT_PAUSE_U4_W {
        PCNT_CNT_PAUSE_U4_W { w: self }
    }
    #[doc = "Bit 8 - Set this bit to clear unit4's counter."]
    #[inline(always)]
    pub fn pcnt_plus_cnt_rst_u4(&mut self) -> PCNT_PLUS_CNT_RST_U4_W {
        PCNT_PLUS_CNT_RST_U4_W { w: self }
    }
    #[doc = "Bit 7 - Set this bit to pause unit3's counter."]
    #[inline(always)]
    pub fn pcnt_cnt_pause_u3(&mut self) -> PCNT_CNT_PAUSE_U3_W {
        PCNT_CNT_PAUSE_U3_W { w: self }
    }
    #[doc = "Bit 6 - Set this bit to clear unit3's counter."]
    #[inline(always)]
    pub fn pcnt_plus_cnt_rst_u3(&mut self) -> PCNT_PLUS_CNT_RST_U3_W {
        PCNT_PLUS_CNT_RST_U3_W { w: self }
    }
    #[doc = "Bit 5 - Set this bit to pause unit2's counter."]
    #[inline(always)]
    pub fn pcnt_cnt_pause_u2(&mut self) -> PCNT_CNT_PAUSE_U2_W {
        PCNT_CNT_PAUSE_U2_W { w: self }
    }
    #[doc = "Bit 4 - Set this bit to clear unit2's counter."]
    #[inline(always)]
    pub fn pcnt_plus_cnt_rst_u2(&mut self) -> PCNT_PLUS_CNT_RST_U2_W {
        PCNT_PLUS_CNT_RST_U2_W { w: self }
    }
    #[doc = "Bit 3 - Set this bit to pause unit1's counter."]
    #[inline(always)]
    pub fn pcnt_cnt_pause_u1(&mut self) -> PCNT_CNT_PAUSE_U1_W {
        PCNT_CNT_PAUSE_U1_W { w: self }
    }
    #[doc = "Bit 2 - Set this bit to clear unit1's counter."]
    #[inline(always)]
    pub fn pcnt_plus_cnt_rst_u1(&mut self) -> PCNT_PLUS_CNT_RST_U1_W {
        PCNT_PLUS_CNT_RST_U1_W { w: self }
    }
    #[doc = "Bit 1 - Set this bit to pause unit0's counter."]
    #[inline(always)]
    pub fn pcnt_cnt_pause_u0(&mut self) -> PCNT_CNT_PAUSE_U0_W {
        PCNT_CNT_PAUSE_U0_W { w: self }
    }
    #[doc = "Bit 0 - Set this bit to clear unit0's counter."]
    #[inline(always)]
    pub fn pcnt_plus_cnt_rst_u0(&mut self) -> PCNT_PLUS_CNT_RST_U0_W {
        PCNT_PLUS_CNT_RST_U0_W { w: self }
    }
}
