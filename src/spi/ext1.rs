#[doc = "Reader of register EXT1"]
pub type R = crate::R<u32, super::EXT1>;
#[doc = "Writer for register EXT1"]
pub type W = crate::W<u32, super::EXT1>;
#[doc = "Register EXT1 `reset()`'s with value 0"]
impl crate::ResetValue for super::EXT1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `T_ERASE_ENA`"]
pub type T_ERASE_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `T_ERASE_ENA`"]
pub struct T_ERASE_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> T_ERASE_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `T_ERASE_SHIFT`"]
pub type T_ERASE_SHIFT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `T_ERASE_SHIFT`"]
pub struct T_ERASE_SHIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> T_ERASE_SHIFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `T_ERASE_TIME`"]
pub type T_ERASE_TIME_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `T_ERASE_TIME`"]
pub struct T_ERASE_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> T_ERASE_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - erase flash delay enable."]
    #[inline(always)]
    pub fn t_erase_ena(&self) -> T_ERASE_ENA_R {
        T_ERASE_ENA_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - erase flash delay time shift."]
    #[inline(always)]
    pub fn t_erase_shift(&self) -> T_ERASE_SHIFT_R {
        T_ERASE_SHIFT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 0:11 - erase flash delay time by system clock."]
    #[inline(always)]
    pub fn t_erase_time(&self) -> T_ERASE_TIME_R {
        T_ERASE_TIME_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - erase flash delay enable."]
    #[inline(always)]
    pub fn t_erase_ena(&mut self) -> T_ERASE_ENA_W {
        T_ERASE_ENA_W { w: self }
    }
    #[doc = "Bits 16:19 - erase flash delay time shift."]
    #[inline(always)]
    pub fn t_erase_shift(&mut self) -> T_ERASE_SHIFT_W {
        T_ERASE_SHIFT_W { w: self }
    }
    #[doc = "Bits 0:11 - erase flash delay time by system clock."]
    #[inline(always)]
    pub fn t_erase_time(&mut self) -> T_ERASE_TIME_W {
        T_ERASE_TIME_W { w: self }
    }
}
