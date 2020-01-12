#[doc = "Reader of register SDA_SAMPLE"]
pub type R = crate::R<u32, super::SDA_SAMPLE>;
#[doc = "Writer for register SDA_SAMPLE"]
pub type W = crate::W<u32, super::SDA_SAMPLE>;
#[doc = "Register SDA_SAMPLE `reset()`'s with value 0"]
impl crate::ResetValue for super::SDA_SAMPLE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SDA_SAMPLE_TIME`"]
pub type SDA_SAMPLE_TIME_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SDA_SAMPLE_TIME`"]
pub struct SDA_SAMPLE_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> SDA_SAMPLE_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - This register is used to configure the clock num I2C used to sample data on SDA after the posedge of SCL"]
    #[inline(always)]
    pub fn sda_sample_time(&self) -> SDA_SAMPLE_TIME_R {
        SDA_SAMPLE_TIME_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - This register is used to configure the clock num I2C used to sample data on SDA after the posedge of SCL"]
    #[inline(always)]
    pub fn sda_sample_time(&mut self) -> SDA_SAMPLE_TIME_W {
        SDA_SAMPLE_TIME_W { w: self }
    }
}
