#[doc = "Reader of register PIN38"]
pub type R = crate::R<u32, super::PIN38>;
#[doc = "Writer for register PIN38"]
pub type W = crate::W<u32, super::PIN38>;
#[doc = "Register PIN38 `reset()`'s with value 0"]
impl crate::ResetValue for super::PIN38 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PIN38_INT_ENA`"]
pub type PIN38_INT_ENA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PIN38_INT_ENA`"]
pub struct PIN38_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN38_INT_ENA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 13)) | (((value as u32) & 0x1f) << 13);
        self.w
    }
}
#[doc = "Reader of field `PIN38_CONFIG`"]
pub type PIN38_CONFIG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PIN38_CONFIG`"]
pub struct PIN38_CONFIG_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN38_CONFIG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
        self.w
    }
}
#[doc = "Reader of field `PIN38_WAKEUP_ENABLE`"]
pub type PIN38_WAKEUP_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PIN38_WAKEUP_ENABLE`"]
pub struct PIN38_WAKEUP_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN38_WAKEUP_ENABLE_W<'a> {
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
#[doc = "Reader of field `PIN38_INT_TYPE`"]
pub type PIN38_INT_TYPE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PIN38_INT_TYPE`"]
pub struct PIN38_INT_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN38_INT_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 7)) | (((value as u32) & 0x07) << 7);
        self.w
    }
}
#[doc = "Reader of field `PIN38_PAD_DRIVER`"]
pub type PIN38_PAD_DRIVER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PIN38_PAD_DRIVER`"]
pub struct PIN38_PAD_DRIVER_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN38_PAD_DRIVER_W<'a> {
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
impl R {
    #[doc = "Bits 13:17 - bit0: APP CPU interrupt enable bit1: APP CPU non-maskable interrupt enable bit3: PRO CPU interrupt enable bit4: PRO CPU non-maskable interrupt enable bit5: SDIO's extent interrupt enable"]
    #[inline(always)]
    pub fn pin38_int_ena(&self) -> PIN38_INT_ENA_R {
        PIN38_INT_ENA_R::new(((self.bits >> 13) & 0x1f) as u8)
    }
    #[doc = "Bits 11:12 - NA"]
    #[inline(always)]
    pub fn pin38_config(&self) -> PIN38_CONFIG_R {
        PIN38_CONFIG_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bit 10 - GPIO wake up enable only available in light sleep"]
    #[inline(always)]
    pub fn pin38_wakeup_enable(&self) -> PIN38_WAKEUP_ENABLE_R {
        PIN38_WAKEUP_ENABLE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 7:9 - if set to 0: GPIO interrupt disable if set to 1: rising edge trigger if set to 2: falling edge trigger if set to 3: any edge trigger if set to 4: low level trigger if set to 5: high level trigger"]
    #[inline(always)]
    pub fn pin38_int_type(&self) -> PIN38_INT_TYPE_R {
        PIN38_INT_TYPE_R::new(((self.bits >> 7) & 0x07) as u8)
    }
    #[doc = "Bit 2 - if set to 0: normal output if set to 1: open drain"]
    #[inline(always)]
    pub fn pin38_pad_driver(&self) -> PIN38_PAD_DRIVER_R {
        PIN38_PAD_DRIVER_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 13:17 - bit0: APP CPU interrupt enable bit1: APP CPU non-maskable interrupt enable bit3: PRO CPU interrupt enable bit4: PRO CPU non-maskable interrupt enable bit5: SDIO's extent interrupt enable"]
    #[inline(always)]
    pub fn pin38_int_ena(&mut self) -> PIN38_INT_ENA_W {
        PIN38_INT_ENA_W { w: self }
    }
    #[doc = "Bits 11:12 - NA"]
    #[inline(always)]
    pub fn pin38_config(&mut self) -> PIN38_CONFIG_W {
        PIN38_CONFIG_W { w: self }
    }
    #[doc = "Bit 10 - GPIO wake up enable only available in light sleep"]
    #[inline(always)]
    pub fn pin38_wakeup_enable(&mut self) -> PIN38_WAKEUP_ENABLE_W {
        PIN38_WAKEUP_ENABLE_W { w: self }
    }
    #[doc = "Bits 7:9 - if set to 0: GPIO interrupt disable if set to 1: rising edge trigger if set to 2: falling edge trigger if set to 3: any edge trigger if set to 4: low level trigger if set to 5: high level trigger"]
    #[inline(always)]
    pub fn pin38_int_type(&mut self) -> PIN38_INT_TYPE_W {
        PIN38_INT_TYPE_W { w: self }
    }
    #[doc = "Bit 2 - if set to 0: normal output if set to 1: open drain"]
    #[inline(always)]
    pub fn pin38_pad_driver(&mut self) -> PIN38_PAD_DRIVER_W {
        PIN38_PAD_DRIVER_W { w: self }
    }
}
