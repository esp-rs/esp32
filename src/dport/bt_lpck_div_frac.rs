#[doc = "Reader of register BT_LPCK_DIV_FRAC"]
pub type R = crate::R<u32, super::BT_LPCK_DIV_FRAC>;
#[doc = "Writer for register BT_LPCK_DIV_FRAC"]
pub type W = crate::W<u32, super::BT_LPCK_DIV_FRAC>;
#[doc = "Register BT_LPCK_DIV_FRAC `reset()`'s with value 0"]
impl crate::ResetValue for super::BT_LPCK_DIV_FRAC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LPCLK_SEL_XTAL32K`"]
pub type LPCLK_SEL_XTAL32K_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPCLK_SEL_XTAL32K`"]
pub struct LPCLK_SEL_XTAL32K_W<'a> {
    w: &'a mut W,
}
impl<'a> LPCLK_SEL_XTAL32K_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `LPCLK_SEL_XTAL`"]
pub type LPCLK_SEL_XTAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPCLK_SEL_XTAL`"]
pub struct LPCLK_SEL_XTAL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPCLK_SEL_XTAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `LPCLK_SEL_8M`"]
pub type LPCLK_SEL_8M_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPCLK_SEL_8M`"]
pub struct LPCLK_SEL_8M_W<'a> {
    w: &'a mut W,
}
impl<'a> LPCLK_SEL_8M_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `LPCLK_SEL_RTC_SLOW`"]
pub type LPCLK_SEL_RTC_SLOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPCLK_SEL_RTC_SLOW`"]
pub struct LPCLK_SEL_RTC_SLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> LPCLK_SEL_RTC_SLOW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `BT_LPCK_DIV_A`"]
pub type BT_LPCK_DIV_A_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BT_LPCK_DIV_A`"]
pub struct BT_LPCK_DIV_A_W<'a> {
    w: &'a mut W,
}
impl<'a> BT_LPCK_DIV_A_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 12)) | (((value as u32) & 0x0fff) << 12);
        self.w
    }
}
#[doc = "Reader of field `BT_LPCK_DIV_B`"]
pub type BT_LPCK_DIV_B_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BT_LPCK_DIV_B`"]
pub struct BT_LPCK_DIV_B_W<'a> {
    w: &'a mut W,
}
impl<'a> BT_LPCK_DIV_B_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn lpclk_sel_xtal32k(&self) -> LPCLK_SEL_XTAL32K_R {
        LPCLK_SEL_XTAL32K_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn lpclk_sel_xtal(&self) -> LPCLK_SEL_XTAL_R {
        LPCLK_SEL_XTAL_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn lpclk_sel_8m(&self) -> LPCLK_SEL_8M_R {
        LPCLK_SEL_8M_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn lpclk_sel_rtc_slow(&self) -> LPCLK_SEL_RTC_SLOW_R {
        LPCLK_SEL_RTC_SLOW_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 12:23"]
    #[inline(always)]
    pub fn bt_lpck_div_a(&self) -> BT_LPCK_DIV_A_R {
        BT_LPCK_DIV_A_R::new(((self.bits >> 12) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn bt_lpck_div_b(&self) -> BT_LPCK_DIV_B_R {
        BT_LPCK_DIV_B_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn lpclk_sel_xtal32k(&mut self) -> LPCLK_SEL_XTAL32K_W {
        LPCLK_SEL_XTAL32K_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn lpclk_sel_xtal(&mut self) -> LPCLK_SEL_XTAL_W {
        LPCLK_SEL_XTAL_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn lpclk_sel_8m(&mut self) -> LPCLK_SEL_8M_W {
        LPCLK_SEL_8M_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn lpclk_sel_rtc_slow(&mut self) -> LPCLK_SEL_RTC_SLOW_W {
        LPCLK_SEL_RTC_SLOW_W { w: self }
    }
    #[doc = "Bits 12:23"]
    #[inline(always)]
    pub fn bt_lpck_div_a(&mut self) -> BT_LPCK_DIV_A_W {
        BT_LPCK_DIV_A_W { w: self }
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn bt_lpck_div_b(&mut self) -> BT_LPCK_DIV_B_W {
        BT_LPCK_DIV_B_W { w: self }
    }
}
