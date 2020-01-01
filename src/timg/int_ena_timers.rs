#[doc = "Reader of register INT_ENA_TIMERS"]
pub type R = crate::R<u32, super::INT_ENA_TIMERS>;
#[doc = "Writer for register INT_ENA_TIMERS"]
pub type W = crate::W<u32, super::INT_ENA_TIMERS>;
#[doc = "Register INT_ENA_TIMERS `reset()`'s with value 0"]
impl crate::ResetValue for super::INT_ENA_TIMERS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIMG_LACT_INT_ENA`"]
pub type TIMG_LACT_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMG_LACT_INT_ENA`"]
pub struct TIMG_LACT_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_LACT_INT_ENA_W<'a> {
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
#[doc = "Reader of field `TIMG_WDT_INT_ENA`"]
pub type TIMG_WDT_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMG_WDT_INT_ENA`"]
pub struct TIMG_WDT_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_WDT_INT_ENA_W<'a> {
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
#[doc = "Reader of field `TIMG_T1_INT_ENA`"]
pub type TIMG_T1_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMG_T1_INT_ENA`"]
pub struct TIMG_T1_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_T1_INT_ENA_W<'a> {
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
#[doc = "Reader of field `TIMG_T0_INT_ENA`"]
pub type TIMG_T0_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMG_T0_INT_ENA`"]
pub struct TIMG_T0_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_T0_INT_ENA_W<'a> {
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
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn timg_lact_int_ena(&self) -> TIMG_LACT_INT_ENA_R {
        TIMG_LACT_INT_ENA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt when an interrupt stage timeout"]
    #[inline(always)]
    pub fn timg_wdt_int_ena(&self) -> TIMG_WDT_INT_ENA_R {
        TIMG_WDT_INT_ENA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - interrupt when timer1 alarm"]
    #[inline(always)]
    pub fn timg_t1_int_ena(&self) -> TIMG_T1_INT_ENA_R {
        TIMG_T1_INT_ENA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - interrupt when timer0 alarm"]
    #[inline(always)]
    pub fn timg_t0_int_ena(&self) -> TIMG_T0_INT_ENA_R {
        TIMG_T0_INT_ENA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn timg_lact_int_ena(&mut self) -> TIMG_LACT_INT_ENA_W {
        TIMG_LACT_INT_ENA_W { w: self }
    }
    #[doc = "Bit 2 - Interrupt when an interrupt stage timeout"]
    #[inline(always)]
    pub fn timg_wdt_int_ena(&mut self) -> TIMG_WDT_INT_ENA_W {
        TIMG_WDT_INT_ENA_W { w: self }
    }
    #[doc = "Bit 1 - interrupt when timer1 alarm"]
    #[inline(always)]
    pub fn timg_t1_int_ena(&mut self) -> TIMG_T1_INT_ENA_W {
        TIMG_T1_INT_ENA_W { w: self }
    }
    #[doc = "Bit 0 - interrupt when timer0 alarm"]
    #[inline(always)]
    pub fn timg_t0_int_ena(&mut self) -> TIMG_T0_INT_ENA_W {
        TIMG_T0_INT_ENA_W { w: self }
    }
}
