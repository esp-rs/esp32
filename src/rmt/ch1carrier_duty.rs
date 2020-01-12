#[doc = "Reader of register CH1CARRIER_DUTY"]
pub type R = crate::R<u32, super::CH1CARRIER_DUTY>;
#[doc = "Writer for register CH1CARRIER_DUTY"]
pub type W = crate::W<u32, super::CH1CARRIER_DUTY>;
#[doc = "Register CH1CARRIER_DUTY `reset()`'s with value 0"]
impl crate::ResetValue for super::CH1CARRIER_DUTY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CARRIER_HIGH_CH1`"]
pub type CARRIER_HIGH_CH1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CARRIER_HIGH_CH1`"]
pub struct CARRIER_HIGH_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> CARRIER_HIGH_CH1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `CARRIER_LOW_CH1`"]
pub type CARRIER_LOW_CH1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CARRIER_LOW_CH1`"]
pub struct CARRIER_LOW_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> CARRIER_LOW_CH1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - This register is used to configure carrier wave's high level value for channel1."]
    #[inline(always)]
    pub fn carrier_high_ch1(&self) -> CARRIER_HIGH_CH1_R {
        CARRIER_HIGH_CH1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - This register is used to configure carrier wave's low level value for channel1."]
    #[inline(always)]
    pub fn carrier_low_ch1(&self) -> CARRIER_LOW_CH1_R {
        CARRIER_LOW_CH1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - This register is used to configure carrier wave's high level value for channel1."]
    #[inline(always)]
    pub fn carrier_high_ch1(&mut self) -> CARRIER_HIGH_CH1_W {
        CARRIER_HIGH_CH1_W { w: self }
    }
    #[doc = "Bits 0:15 - This register is used to configure carrier wave's low level value for channel1."]
    #[inline(always)]
    pub fn carrier_low_ch1(&mut self) -> CARRIER_LOW_CH1_W {
        CARRIER_LOW_CH1_W { w: self }
    }
}
