#[doc = "Reader of register OUT1_W1TC"]
pub type R = crate::R<u32, super::OUT1_W1TC>;
#[doc = "Writer for register OUT1_W1TC"]
pub type W = crate::W<u32, super::OUT1_W1TC>;
#[doc = "Register OUT1_W1TC `reset()`'s with value 0"]
impl crate::ResetValue for super::OUT1_W1TC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OUT1_DATA_W1TC`"]
pub type OUT1_DATA_W1TC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OUT1_DATA_W1TC`"]
pub struct OUT1_DATA_W1TC_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT1_DATA_W1TC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - GPIO32~39 output value write 1 to clear"]
    #[inline(always)]
    pub fn out1_data_w1tc(&self) -> OUT1_DATA_W1TC_R {
        OUT1_DATA_W1TC_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO32~39 output value write 1 to clear"]
    #[inline(always)]
    pub fn out1_data_w1tc(&mut self) -> OUT1_DATA_W1TC_W {
        OUT1_DATA_W1TC_W { w: self }
    }
}
