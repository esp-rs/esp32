#[doc = "Reader of register INT_ST_TIMERS"]
pub type R = crate::R<u32, super::INT_ST_TIMERS>;
#[doc = "Writer for register INT_ST_TIMERS"]
pub type W = crate::W<u32, super::INT_ST_TIMERS>;
#[doc = "Register INT_ST_TIMERS `reset()`'s with value 0"]
impl crate::ResetValue for super::INT_ST_TIMERS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LACT_INT_ST`"]
pub type LACT_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LACT_INT_ST`"]
pub struct LACT_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> LACT_INT_ST_W<'a> {
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
#[doc = "Reader of field `WDT_INT_ST`"]
pub type WDT_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDT_INT_ST`"]
pub struct WDT_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_INT_ST_W<'a> {
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
#[doc = "Reader of field `T1_INT_ST`"]
pub type T1_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `T1_INT_ST`"]
pub struct T1_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> T1_INT_ST_W<'a> {
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
#[doc = "Reader of field `T0_INT_ST`"]
pub type T0_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `T0_INT_ST`"]
pub struct T0_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> T0_INT_ST_W<'a> {
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
    pub fn lact_int_st(&self) -> LACT_INT_ST_R {
        LACT_INT_ST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt when an interrupt stage timeout"]
    #[inline(always)]
    pub fn wdt_int_st(&self) -> WDT_INT_ST_R {
        WDT_INT_ST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - interrupt when timer1 alarm"]
    #[inline(always)]
    pub fn t1_int_st(&self) -> T1_INT_ST_R {
        T1_INT_ST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - interrupt when timer0 alarm"]
    #[inline(always)]
    pub fn t0_int_st(&self) -> T0_INT_ST_R {
        T0_INT_ST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn lact_int_st(&mut self) -> LACT_INT_ST_W {
        LACT_INT_ST_W { w: self }
    }
    #[doc = "Bit 2 - Interrupt when an interrupt stage timeout"]
    #[inline(always)]
    pub fn wdt_int_st(&mut self) -> WDT_INT_ST_W {
        WDT_INT_ST_W { w: self }
    }
    #[doc = "Bit 1 - interrupt when timer1 alarm"]
    #[inline(always)]
    pub fn t1_int_st(&mut self) -> T1_INT_ST_W {
        T1_INT_ST_W { w: self }
    }
    #[doc = "Bit 0 - interrupt when timer0 alarm"]
    #[inline(always)]
    pub fn t0_int_st(&mut self) -> T0_INT_ST_W {
        T0_INT_ST_W { w: self }
    }
}
