#[doc = "Reader of register QUICK_SENT"]
pub type R = crate::R<u32, super::QUICK_SENT>;
#[doc = "Writer for register QUICK_SENT"]
pub type W = crate::W<u32, super::QUICK_SENT>;
#[doc = "Register QUICK_SENT `reset()`'s with value 0"]
impl crate::ResetValue for super::QUICK_SENT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ALWAYS_SEND_EN`"]
pub type ALWAYS_SEND_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALWAYS_SEND_EN`"]
pub struct ALWAYS_SEND_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALWAYS_SEND_EN_W<'a> {
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
#[doc = "Reader of field `ALWAYS_SEND_NUM`"]
pub type ALWAYS_SEND_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ALWAYS_SEND_NUM`"]
pub struct ALWAYS_SEND_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> ALWAYS_SEND_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `SINGLE_SEND_EN`"]
pub type SINGLE_SEND_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SINGLE_SEND_EN`"]
pub struct SINGLE_SEND_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGLE_SEND_EN_W<'a> {
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
#[doc = "Reader of field `SINGLE_SEND_NUM`"]
pub type SINGLE_SEND_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SINGLE_SEND_NUM`"]
pub struct SINGLE_SEND_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGLE_SEND_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bit 7 - Set this bit to enable continuously send the same short packet"]
    #[inline(always)]
    pub fn always_send_en(&self) -> ALWAYS_SEND_EN_R {
        ALWAYS_SEND_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - The bits are used to choose which short packet"]
    #[inline(always)]
    pub fn always_send_num(&self) -> ALWAYS_SEND_NUM_R {
        ALWAYS_SEND_NUM_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 3 - Set this bit to enable send a short packet"]
    #[inline(always)]
    pub fn single_send_en(&self) -> SINGLE_SEND_EN_R {
        SINGLE_SEND_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:2 - The bits are used to choose which short packet"]
    #[inline(always)]
    pub fn single_send_num(&self) -> SINGLE_SEND_NUM_R {
        SINGLE_SEND_NUM_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 7 - Set this bit to enable continuously send the same short packet"]
    #[inline(always)]
    pub fn always_send_en(&mut self) -> ALWAYS_SEND_EN_W {
        ALWAYS_SEND_EN_W { w: self }
    }
    #[doc = "Bits 4:6 - The bits are used to choose which short packet"]
    #[inline(always)]
    pub fn always_send_num(&mut self) -> ALWAYS_SEND_NUM_W {
        ALWAYS_SEND_NUM_W { w: self }
    }
    #[doc = "Bit 3 - Set this bit to enable send a short packet"]
    #[inline(always)]
    pub fn single_send_en(&mut self) -> SINGLE_SEND_EN_W {
        SINGLE_SEND_EN_W { w: self }
    }
    #[doc = "Bits 0:2 - The bits are used to choose which short packet"]
    #[inline(always)]
    pub fn single_send_num(&mut self) -> SINGLE_SEND_NUM_W {
        SINGLE_SEND_NUM_W { w: self }
    }
}
