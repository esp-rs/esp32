#[doc = "Reader of register BT_LPCK_DIV_INT"]
pub type R = crate::R<u32, super::BT_LPCK_DIV_INT>;
#[doc = "Writer for register BT_LPCK_DIV_INT"]
pub type W = crate::W<u32, super::BT_LPCK_DIV_INT>;
#[doc = "Register BT_LPCK_DIV_INT `reset()`'s with value 0"]
impl crate::ResetValue for super::BT_LPCK_DIV_INT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BTEXTWAKEUP_REQ`"]
pub type BTEXTWAKEUP_REQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BTEXTWAKEUP_REQ`"]
pub struct BTEXTWAKEUP_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> BTEXTWAKEUP_REQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `BT_LPCK_DIV_NUM`"]
pub type BT_LPCK_DIV_NUM_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BT_LPCK_DIV_NUM`"]
pub struct BT_LPCK_DIV_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> BT_LPCK_DIV_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn btextwakeup_req(&self) -> BTEXTWAKEUP_REQ_R {
        BTEXTWAKEUP_REQ_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn bt_lpck_div_num(&self) -> BT_LPCK_DIV_NUM_R {
        BT_LPCK_DIV_NUM_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn btextwakeup_req(&mut self) -> BTEXTWAKEUP_REQ_W {
        BTEXTWAKEUP_REQ_W { w: self }
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn bt_lpck_div_num(&mut self) -> BT_LPCK_DIV_NUM_W {
        BT_LPCK_DIV_NUM_W { w: self }
    }
}
