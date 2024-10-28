#[doc = "Register `SERRES` reader"]
pub type R = crate::R<SerresSpec>;
#[doc = "Register `SERRES` writer"]
pub type W = crate::W<SerresSpec>;
#[doc = "Resistor value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Resvalue {
    #[doc = "0: No series resistor"]
    Res0 = 0,
    #[doc = "1: 20 kiloohm series resistor"]
    Res20k = 1,
    #[doc = "2: 50 kiloohm series resistor"]
    Res50k = 2,
    #[doc = "3: 100 kiloohm series resistor"]
    Res100k = 3,
}
impl From<Resvalue> for u8 {
    #[inline(always)]
    fn from(variant: Resvalue) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Resvalue {
    type Ux = u8;
}
impl crate::IsEnum for Resvalue {}
#[doc = "Field `RESISTOR` reader - Resistor value"]
pub type ResistorR = crate::FieldReader<Resvalue>;
impl ResistorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Resvalue {
        match self.bits {
            0 => Resvalue::Res0,
            1 => Resvalue::Res20k,
            2 => Resvalue::Res50k,
            3 => Resvalue::Res100k,
            _ => unreachable!(),
        }
    }
    #[doc = "No series resistor"]
    #[inline(always)]
    pub fn is_res0(&self) -> bool {
        *self == Resvalue::Res0
    }
    #[doc = "20 kiloohm series resistor"]
    #[inline(always)]
    pub fn is_res20k(&self) -> bool {
        *self == Resvalue::Res20k
    }
    #[doc = "50 kiloohm series resistor"]
    #[inline(always)]
    pub fn is_res50k(&self) -> bool {
        *self == Resvalue::Res50k
    }
    #[doc = "100 kiloohm series resistor"]
    #[inline(always)]
    pub fn is_res100k(&self) -> bool {
        *self == Resvalue::Res100k
    }
}
#[doc = "Field `RESISTOR` writer - Resistor value"]
pub type ResistorW<'a, REG> = crate::FieldWriter<'a, REG, 2, Resvalue, crate::Safe>;
impl<'a, REG> ResistorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No series resistor"]
    #[inline(always)]
    pub fn res0(self) -> &'a mut crate::W<REG> {
        self.variant(Resvalue::Res0)
    }
    #[doc = "20 kiloohm series resistor"]
    #[inline(always)]
    pub fn res20k(self) -> &'a mut crate::W<REG> {
        self.variant(Resvalue::Res20k)
    }
    #[doc = "50 kiloohm series resistor"]
    #[inline(always)]
    pub fn res50k(self) -> &'a mut crate::W<REG> {
        self.variant(Resvalue::Res50k)
    }
    #[doc = "100 kiloohm series resistor"]
    #[inline(always)]
    pub fn res100k(self) -> &'a mut crate::W<REG> {
        self.variant(Resvalue::Res100k)
    }
}
impl R {
    #[doc = "Bits 0:1 - Resistor value"]
    #[inline(always)]
    pub fn resistor(&self) -> ResistorR {
        ResistorR::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Resistor value"]
    #[inline(always)]
    #[must_use]
    pub fn resistor(&mut self) -> ResistorW<SerresSpec> {
        ResistorW::new(self, 0)
    }
}
#[doc = "Series resistor for PTC measurements\n\nYou can [`read`](crate::Reg::read) this register and get [`serres::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`serres::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SerresSpec;
impl crate::RegisterSpec for SerresSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`serres::R`](R) reader structure"]
impl crate::Readable for SerresSpec {}
#[doc = "`write(|w| ..)` method takes [`serres::W`](W) writer structure"]
impl crate::Writable for SerresSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SERRES to value 0"]
impl crate::Resettable for SerresSpec {
    const RESET_VALUE: u8 = 0;
}
