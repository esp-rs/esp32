#[doc = "Reader of register SLAVE_ADDR"]
pub type R = crate::R<u32, super::SLAVE_ADDR>;
#[doc = "Writer for register SLAVE_ADDR"]
pub type W = crate::W<u32, super::SLAVE_ADDR>;
#[doc = "Register SLAVE_ADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::SLAVE_ADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADDR_10BIT_EN`"]
pub type ADDR_10BIT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADDR_10BIT_EN`"]
pub struct ADDR_10BIT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_10BIT_EN_W<'a> {
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
#[doc = "Reader of field `SLAVE_ADDR`"]
pub type SLAVE_ADDR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SLAVE_ADDR`"]
pub struct SLAVE_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | ((value as u32) & 0x7fff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - This register is used to enable slave 10bit address mode."]
    #[inline(always)]
    pub fn addr_10bit_en(&self) -> ADDR_10BIT_EN_R {
        ADDR_10BIT_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 0:14 - when configured as i2c slave this register is used to configure slave's address."]
    #[inline(always)]
    pub fn slave_addr(&self) -> SLAVE_ADDR_R {
        SLAVE_ADDR_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - This register is used to enable slave 10bit address mode."]
    #[inline(always)]
    pub fn addr_10bit_en(&mut self) -> ADDR_10BIT_EN_W {
        ADDR_10BIT_EN_W { w: self }
    }
    #[doc = "Bits 0:14 - when configured as i2c slave this register is used to configure slave's address."]
    #[inline(always)]
    pub fn slave_addr(&mut self) -> SLAVE_ADDR_W {
        SLAVE_ADDR_W { w: self }
    }
}
