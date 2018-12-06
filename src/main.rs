mod day_1_2017;
mod day_2_2017;
mod day_3_2017;
mod day_4_2017;
mod day_5_2017;
mod day_6_2017;
mod day_7_2017;


use crate::day_1_2017::day_1_2017;
use crate::day_2_2017::{day_2_2017, day_2_2_2017};
use crate::day_3_2017::{day_3_1_2017, get_distance_for_number, get_distance_with_spiral_struct, day_3_2_2017};
use crate::day_4_2017::{day_4_1_2017, day_4_2_2017};
use crate::day_5_2017::{day_5_1_2017, day_5_2_2017};
use crate::day_6_2017::{day_6_1_2017, day_6_2_2017};


fn run_advent_of_code_2017() {
    println!("--- Advent of Code 2017 ---");

    let day_1_part_1_vals = "77736991856689225253142335214746294932318813454849177823468674346512426482777696993348135287531487622845155339235443718798255411492778415157351753377959586612882455464736285648473397681163729345143319577258292849619491486748832944425643737899293811819448271546283914592546989275992844383947572926628695617661344293284789225493932487897149244685921644561896799491668147588536732985476538413354195246785378443492137893161362862587297219368699689318441563683292683855151652394244688119527728613756153348584975372656877565662527436152551476175644428333449297581939357656843784849965764796365272113837436618857363585783813291999774718355479485961244782148994281845717611589612672436243788252212252489833952785291284935439662751339273847424621193587955284885915987692812313251556836958571335334281322495251889724281863765636441971178795365413267178792118544937392522893132283573129821178591214594778712292228515169348771198167462495988252456944269678515277886142827218825358561772588377998394984947946121983115158951297156321289231481348126998584455974277123213413359859659339792627742476688827577318285573236187838749444212666293172899385531383551142896847178342163129883523694183388123567744916752899386265368245342587281521723872555392212596227684414269667696229995976182762587281829533181925696289733325513618571116199419759821597197636415243789757789129824537812428338192536462468554399548893532588928486825398895911533744671691387494516395641555683144968644717265849634943691721391779987198764147667349266877149238695714118982841721323853294642175381514347345237721288281254828745122878268792661867994785585131534136646954347165597315643658739688567246339618795777125767432162928257331951255792438831957359141651634491912746875748363394329848227391812251812842263277229514125426682179711184717737714178235995431465217547759282779499842892993556918977773236196185348965713241211365895519697294982523166196268941976859987925578945185217127344619169353395993198368185217391883839449331638641744279836858188235296951745922667612379649453277174224722894599153367373494255388826855322712652812127873536473277";
    println!("    * Day 1 - Part 1 ==> {}", day_1_2017(day_1_part_1_vals, 1));

    let day_1_part_2_vals = "77736991856689225253142335214746294932318813454849177823468674346512426482777696993348135287531487622845155339235443718798255411492778415157351753377959586612882455464736285648473397681163729345143319577258292849619491486748832944425643737899293811819448271546283914592546989275992844383947572926628695617661344293284789225493932487897149244685921644561896799491668147588536732985476538413354195246785378443492137893161362862587297219368699689318441563683292683855151652394244688119527728613756153348584975372656877565662527436152551476175644428333449297581939357656843784849965764796365272113837436618857363585783813291999774718355479485961244782148994281845717611589612672436243788252212252489833952785291284935439662751339273847424621193587955284885915987692812313251556836958571335334281322495251889724281863765636441971178795365413267178792118544937392522893132283573129821178591214594778712292228515169348771198167462495988252456944269678515277886142827218825358561772588377998394984947946121983115158951297156321289231481348126998584455974277123213413359859659339792627742476688827577318285573236187838749444212666293172899385531383551142896847178342163129883523694183388123567744916752899386265368245342587281521723872555392212596227684414269667696229995976182762587281829533181925696289733325513618571116199419759821597197636415243789757789129824537812428338192536462468554399548893532588928486825398895911533744671691387494516395641555683144968644717265849634943691721391779987198764147667349266877149238695714118982841721323853294642175381514347345237721288281254828745122878268792661867994785585131534136646954347165597315643658739688567246339618795777125767432162928257331951255792438831957359141651634491912746875748363394329848227391812251812842263277229514125426682179711184717737714178235995431465217547759282779499842892993556918977773236196185348965713241211365895519697294982523166196268941976859987925578945185217127344619169353395993198368185217391883839449331638641744279836858188235296951745922667612379649453277174224722894599153367373494255388826855322712652812127873536473277";

    println!("    * Day 1 - Part 2 ==> {}", day_1_2017(day_1_part_2_vals, day_1_part_2_vals.len() / 2));

    let val = "790	99	345	1080	32	143	1085	984	553	98	123	97	197	886	125	947
302	463	59	58	55	87	508	54	472	63	469	419	424	331	337	72
899	962	77	1127	62	530	78	880	129	1014	93	148	239	288	357	424
2417	2755	254	3886	5336	3655	5798	3273	5016	178	270	6511	223	5391	1342	2377
68	3002	3307	166	275	1989	1611	364	157	144	3771	1267	3188	3149	156	3454
1088	1261	21	1063	1173	278	1164	207	237	1230	1185	431	232	660	195	1246
49	1100	136	1491	647	1486	112	1278	53	1564	1147	1068	809	1638	138	117
158	3216	1972	2646	3181	785	2937	365	611	1977	1199	2972	201	2432	186	160
244	86	61	38	58	71	243	52	245	264	209	265	308	80	126	129
1317	792	74	111	1721	252	1082	1881	1349	94	891	1458	331	1691	89	1724
3798	202	3140	3468	1486	2073	3872	3190	3481	3760	2876	182	2772	226	3753	188
2272	6876	6759	218	272	4095	4712	6244	4889	2037	234	223	6858	3499	2358	439
792	230	886	824	762	895	99	799	94	110	747	635	91	406	89	157
2074	237	1668	1961	170	2292	2079	1371	1909	221	2039	1022	193	2195	1395	2123
8447	203	1806	6777	278	2850	1232	6369	398	235	212	992	7520	7304	7852	520
3928	107	3406	123	2111	2749	223	125	134	146	3875	1357	508	1534	4002	4417";

    println!("    * Day 2 - Part 1 ==> {:?}", day_2_2017(val));

    println!("    * Day 2 - Part 2 ==> {:?}", day_2_2_2017(val));

    println!("    * Day 3 - Part 1 ==> {}", day_3_1_2017(361_527));

    println!("    * Day 3 - Part 1 (with spiral iter) ==> {}", get_distance_for_number(361_527));

    println!("    * Day 3 - Part 1 (with spiral struct iter impl) ==> {}", get_distance_with_spiral_struct(361_527));

    println!("    * Day 3 - Part 2 ==> {}", day_3_2_2017(361_527));

    let input = "pphsv ojtou brvhsj cer ntfhlra udeh ccgtyzc zoyzmh jum lugbnk
vxjnf fzqitnj uyfck blnl impo kxoow nngd worcm bdesehw
caibh nfuk kfnu llfdbz uxjty yxjut jcea
qiho qif eupwww avyglnj nxzotsu hio lws
xjty usocjsh pivk qnknunc yjcgh bwya djw zpyr
ycfmfe mgq sjiomg nfzjul bjwkmgu yvsnvgj dcjupu wzz blmn
rdowgbt vpwfdoi blzl laghnk gsa vhnpo cztxzlb rtz hvwonhb eciju pfjtbo
bqs bqs dbutvgf mmzb izpyud rap izpyud xlzeb mnj hjncs
xpu vwp nujcos piu irindir tpmfd umtvlm gznu
sfpuxar qcnbte omouazv cnh uaxspfr sepolf rusafpx
xbmaf iceyqqq sabpt gliexel muubepe qqiyqce fmrcc eazk obkeonl fmccr kgk
apg gbycwe gap pag
gagv saqbk lwtllc wnhzz khxsjc
lgc alen rlmsp anel gcbvg
bujlaz rks rlqf deknmee yrp
scqvl weusbc bgvaz vgg cjwsfno vqy zbq aqy tvf bgzav
hbki vei fxdwljs myjuba elbsib pvy xxjxgi dtgv
linzaeu qbwdke fdg pykw
qvtdd aco aav bpu mvkcuc kjfj japgfki jfdl gem hog bdzsiea
wpbigkb lzhwba jssjkn qvb kmwu qddv
iny osyvqnt tumunzb torq bdeneg wywank poza ipp iggorw
tuko mhdbsf vmjdop jomaqpj rcdsud hmgspr lsas nzmwc
cirkjq nmjuu xtgejv gtexvj vjcmtqq unjmu
xsdmezq xvqjvqp exhygy qahju hvd qadmdh lok
wvvys kax rohrrar rwhnvi lhnmefp lsktouy bxilosp
wayf diobnl zvu obnidl oibnld
cewil ygsf ffzp ruxhu vah lnvwt aef lnnjc kgkb gxtlx feko
uti epphrin pywths cpzzh csjei nczhamy gayxmb bdcytq xkx fgmt
qvzyuwi dwo swkw bwjdrn dasgd ijgw vzabaop yefyhmc wgij
dyg sugrf vid etz weyqg nyntx dwfgwm khon hnzzzn xfyra
ofbh bdrsk rdrjj elaxvk jrjdr
msxau rsocvx zxdda mxz lknl
qktaywx dirpdbf unqnd wbrwkuu fvmqwl emxr big
xwz kvsydc ayokjyy qiah omw neo htltxx fxhwqwj colqvbb sxmo ephfkex
ncjxoaf fwjkc czmhv ylg axcjofn dvj bzqjku opvcr jiwzucg vmhzc
gmmnrt zqar twdwrg qiwwki fcbr lixm hjdwwe moiva
roinlxg cxeezve whannk cxeezve pyoj boweioy cpkgxsz
qkct qso xlb xyy aellfet rzt cbboow devfb nih fhbfxzi
qyc ltxia alixt atilx xtgrv
svruz ufvo rvesnxv dik vzurs jjg idk
xeudhrg hudn cilo ljplosb
kpb oyzvywx vldko qhfkwod bkeutk zqcqug pbriu wqocos
qkngzfy whobyri aze jvipdty ocirbep icqwc
kzxxlab sjr zhymws xkbx
nnxs gkwtld dwhkry snuibq dtdl aicug bhtlfzp qzk jctos
regvro mxcq hqof yraucxi jhkol iuxineo pbtnk rfjwc szgjpr ndqqj vfgm
yqrfox xoqrfy utbryu utubyr
jdubjt wqrl wnk rlqw nwiq pnbn qinw uaff ftdo htfrav
rum mur umr tij ovbahl losao imawwpb wadhww tbteyqc
napxd kzeiqcp ppgqucm xkityt frq hugrp gjgtt gmuqppc zwqme
xyuzs ysch howlzgu dkqppbs nvbiz mks mtxv vivouex uvawq
ffe lfsn nlq mpulheq ikcfo wdtz cnwsbph zkib muu
bqkxav wtecb lxwdhr kqbavx aqxvbk
czwswqx ldkxapd pfwd bdkkj iqohla cwosw ihqpd pcc ckhabbn
foiip hau rbqiyhh htm omeubgh symh evfcqg
lqx xlq rsgf izu esetis
npsrkdj fvulgkw eovw mzr uobcze azb tij ihoer ehori jit wknsqhm
gnrksh xwggt oosi bpnmhx qqaa mpmryu jhzyz
yad gbexqcr gbexqcr gbexqcr
ldca xxhznn twyy ytwy zhxnnx xfmpi
floioot kfyh dhibv ezyznar sfg sfg ezyznar
cinioim iiocmin ypla aypl
mhwcjbz dftuqsy wswop eizbf ptsd
ehx mlh nfxgfkz uuw xftmn ptlkbo vsnyo ttwce
oexvf orcg cncnkfk comvhl
lqewsj lyulrcl efixd qvd fhznqnz yvrkwyi xmhgc vzbp
dmr wrxqh thcm giomp rtvl ssc gwq rbklw hcmt fjvud
teozhb dmzwfv qkq pvcqfqq
hvlebc qqmg repxk zwrjdx ztruwb such tyligs ybg
psa rqznokd lgc jstqres yiqt mbiody xazb xjuk dtb
lea ncm rnh myzqzwm
wjml eums ueflvbr cjpgnl qduunu zfxaai jwlm lprzzg vrn ttetyr sume
uwkgeu uiahd plyewgi vveo nwhsitz mcitc uvk zsxehgs sewl
lnbdrka sgtivn sozzq mgd vhxfnlr twrfpk
gadphmk mbx lmlbrf tsnehnr lawdpm fnima gxgl
umty vrn dpow fsnnpjv fsnvnjp nnsvpjf cioaio
euu uue zeskmtk hob stekkzm
ypqpri qwdju ypriqp iprqyp jnoxqa
lkppi ingfxw wlulvp yhwrli nxwigf oyuhq ggfslx
kdd ypvr pyvr waw vyrp khqq mamxca bapq gobfm
iuq upvdpv zxef bfwns lmq lxswr kpsqo pwde iaaou nsw udy
lgzo nil ovgrmt omgtrv jrqp pqrj lit
uumyu iiakfj gvdtzz qbux yxn ejs dvlts
hcm ghutxq zswi tmyhqef hgxtuq
shkhkdk kad seubeax kdl mzu
cpykgr skx rfhpor xsk moyhlai ogv ophfrr dxipuuh
beyw jvrre opodn zdoajhx fhg ijs drczy drczy hjungq
jrzieja gfg yzdn yxm wshibsn fgg
xtylh vxscmvp rfymq uzhpyea spxcmvv dlni msj yxhlt
eov awql miv miv eov
mmvrfbg fjiyf hvqz zpuqmbf fszyuz ldfgni wemfjl fjjpl rbnpy rfb
ppzpeh nam ntv xnchtyk hja hpepzp foj bibvx nmmdlff bsrkp
qiy qiy umhlnh qiy
tyds oepk wae tdsy sdty
ukawr rkwau ghtjhm axy
wtbjiv btjivw ewaf hwk ttq
kdpun myve sqv rhvpy fnjwt puw ujhf thsp nkdadqr
vyw wkkpdpy xlgz lmmcuve ncuq lmotk
pmsfw vxd jpe qxlyasx ejp gwuv
pmgyndm ezofbvx nicbwrw kwnlj yjvnas fdpkfo mqcsyhn pyjpf fbexvzo vkftm erl
trmwvk rywuzoz hbidea kicohfz heidab deaibh
sogf govd dknpk vxrvk rlm vwhjk
xnxbfmw wguzrhd zbmkz piwppa mkbzz xvwrdgy flusfqb
cgduq hbnwr xfx mrejb ckw zkbaihf cloow cwk wuvthv iwqctx
vugx qbucd gxuv ocb cob
ilmet fbelxxz qratdfn unoj hbc duv srmikz
vnzuw zgpbqgf uzm thysyxd dinfh bgvr olungg ksd dsetwqz hpg
omagsf zpr coa kknx bzithq pewp flvoz xiiq weojqr wpep
aagj gcglqt gqcglt xbfx dhdx lbx
pljq plxuscw ilh wfk lhi hli fouieyw
hvnh zvm aqy dzitirm veq ctux
lglhs aqibdii hjbn cfgc qrg pnbntcx owoks ebz
jozngde lwne mbo omb fnyzvvj gndozje
bbdgc igtdj uhahgp sqduko
uuspedu fgnspm ewc slly jbs chl heanm abqijx kadvgxu
akfsft skna kusjqr rkqujs
erc vrljpu lruvjp lpvjur
iors hcdr fsqtcj vop vmn dtqnz tov oscjlw cdrh ctfjsq lrnts
fxp mczo sjlcxa mzoc jmsq hcxybow dmrr bcoxhyw
aac ewraerq odmxpz aac aac
zzio zebmxa szeej poordr gmi owwnnh xfx rzrab lfey jesze
akc yyoj vqod drtne
joxhvyf ymasnbr omouvq isxdrr
qyi ayrkzu jsk vqvvno jkkuxi zufnnwu mrsszdf
ocqi htfb tzjna cdt wkzhynm eergf
yokzugl usyuqu qvotq uweqyow lygkzuo kpmqmb uglyzok
glvshl imqv jrv xlpnsy gcg psj irtiamg wkl
bjcpc nvyloa dkkan efj okubpc cxlowm eone kmpny
cyxqys nmuaftv gqxj gtvsc
beouh dioxiah kizdy hyi cozrray rave fqxmxmj gdm
frjz amrsat lxvhzj azhevtu vxlzhj
zwmnrk sbk txzrcsj sbk oosgfej cvh zuthibi onvwd sbk nhwpzq
gzamt vraw kuk ugayl lyaug bww rwav ijah
bdjirxg vifjr rhbxpa oao yrhjxoi pbn
navb umesiys yhix phuhu aekkciu nlnsiq wjf idqdwp
cmhw rsu urs ziprlfe
kyhxitv cgty bnwjyq cygt sgjn pdab imarvhg yjbnqw
axaa ejancv yau njpc jvwy bpft kwjvg qzrbvtm diu njpc bpft
ambj upe rmqr yudbiqf krudp pqyf
tnb mobnpv vep ohxoc cyip wxyccfo jrbi rwsws kls zlv oohxc
fjh dmb hlbq bqc jhf kax suz fjjg rkpc
wjnn byfirm goeyh xtjmdka
tgyfxx hefpxln mveobqr yeo ftfn srt vim vlcu hevoi xtaaff
imyql xotcl poql rlueapq bkwykm hlalk bkwykm
gkec zff hbmtq rjxjbcf arerlu pvz cdaqi nijmhv uodwjh
mpctof mopftc ksfbat sbkatf
nvdd jub bvi kyggdbx nwtiok gjt mgsm dbhsn rzibgjm dvdn eqi
ysd iirp dfgzza wiyeoou ysd ispkv bcqg wwzqgq xphse
ntq ivposb gsd ezl tlkztp lez qyurp vxsmg dgs
wijs rydbj onm usiyqzb hwrol giusanb kewukl yziuqbs doojam nom
lfacyy xwwast truqtt tzneimn uxsydc ktu eqyaj ndszak
ffleooc kikif fohgop aucy moubqxu
iaxc pnwexdl ncy vmwm xrqoi wpgftq rofx utyzjuf stdxq twpgfq
ppmlp etsvi cjdx poly ynx vfxpslg mqjo qnpsage flpsxvg jwsxiqt
lbyhnb kflrpeq ssoti webxr embbjd kbnx ubzqco
khhc vwuqzb ebocbko rwmonkz edfqn hzh qhncoq gbwdi wjeg ocwow
ghzhd kcxblp lzwkkr gzhdh umk pblcxk
wyajtw jiff ouylv sni lwhlrg avqjiis igzx wbl lhrwgl
glhh kaxha tqii hwzx rgic kaxha rgyidmt qdgxfl ynjc oibfij
bapj bix rjniw ynbql idlvnmt wynpzbl zlpuoix kvn kakwys
aldpxxu iojxp rif xbyqtr jffdvy qnrq tqwsdiu
ulssco ktbymjw bfj zhkg zgc ctyri
ilrmq wfahcgk mrlqi bguad inj
cjzc rekuy ifr wfkg sple
cvjkp qbmumnp mprg ltmwxxh zpemtyb ozzssfd ksu mgrp
nvc sxp mpkxz bhlctq hguaa yrdkm iwsgfg qjssh gobbies hucdh
jdxrjw qmo qmo vobhnu
dnjib wtjp rfdjqdj skpvrb vkwevb kxxovp
fzi kicta zkuvr rfaawv ehklq cfdjsyb tukahwr zkuvr kicta ouq
aba ytdguk gqmpn hvxabff hvxabff dckj
fna wxyqhxd hvy khsu yypoyy lvvue medheua gim slf drdbeh ikihf
jquz wwo wwo ghlz jrbvb jrbvb
jwzvkl yjw ouwla yjw ouwla
zsvlgyf rzqbtj qygynem ukdgjm lbsyh tmdzp fbcaim eymzr
pvw sbs dvsa plmepl pwv ayxk vpw dwt
inayadn pnti yzhxk azga gxq aznbciu gjnmyqm
isgf ndqmk beyqq ebyqq srtzxo aiiw oqfuwp uoqwfp buejctv pxbk
pzl irv tzvzdb wcy eszm ybwiw ycw riizifd iybww
btpu cua azzqffy owcr
ofwq sqlpzat lozdxlc aevjmpc lcolzxd wbbysn qwfo vcrx gdzgi
dbpfmxu ydsxwl ijn svxtop csep ldqeog ffye zcrl soh aclw
wyiyyhv vyhiywy obgi hiyywvy
ddvaoc lhv spurn rgxyy onjw illvn yryxg xyyrg
vid wdttqq kajr myip
wolqlue phlunpt dcmmkfm sgxk dmmckmf sfng jlbsntq dxp
zmneyho fswj xdgsjc oefwjdi htgxvbd tgqrq xodoa
ynw bygqdnh hhmnkuw cojqrke qszzdjo orskwq mdfae asabn
vvpm vkj pcxghao caoxphg axhblxb vvmp
txox nzy eqn zgir dytsi girz ffa ugjjbzj brob fll
kbz pukqbd fiwmuh umwihf bkz dvz
vgs vejs vejs vejs mbkyjjy
viqmnmu bitkyw nddnk dknnd cldnpp hipub plcdpn fdzzpb mmyomn
ndylnfx gozlrx ngptk rnpteb wtacx xmtcjy xldha
fey doyxis ampmtr ycqh syw cqhlj hnngx
dijf nac tvkq ayo akbj lzmngdm wfxpn bpyvrf cvdqpa
zsofz lhho hgat wqskga mnt
mylwm zxsd omzpa waz hcrr lxmpq jsw sqtwak pzoma
rwhgsgt ysdq ztihici mpwcawv alkqg wsxiwx
snldn bcb anjdv cbb awsscc cqxult hjmjew mcycb fdpdg sesrh
kukrqm fawafz qdim wyobtqx bnvjnqg dcvqxta yptr nnpu ughldqp duo zafwaf
knb yjqb bscpnt nzg sqeu zkahna ttuf nsbtpc ixwit vucwj idix
bfqyx xlnpc ijrxu zkqi kjxtahr fgag orusms adi bfqyx bfqyx
dqddc ncbv bvfk hefikb dqddc hqjl otpx zfiu
ntkv qunrzx eztzure ctt rjo bkdt znvd jwdf gqhf mmhrzgt
zeavm hkbf rawqwuf pis dojlkt vnjhmi uvk cufmn qginezd xyut
hnidzk chlctc yst pepd dxntbxg vqk daxfpmu wshyddl
jgd vesqgo bdyqy igl ahstdm wjtd lrtkjsv tjsj sccxbih esn gkkzj
iisiswh jll rhlaf jqwwgfa wmhyo izva vrg zjkak nlxxfer rvhx
mkrkd jlqtpy ukstro ktuors wsj ynqpbp kpiyxzv nxeiwg xpzvkiy
jbr gnct fwklekg cmfqnm ctn gqobrs kwht
pztmjs yiffc kfhsblx yiffc yiffc
biezil iiezbl bzeiil smocoju
viiigm gmmmk yeiv dxzogro qsmzsur hukzwjn lcle syo mdj uruf rxfseu
extchsd adeff ouikoj fyaclr rwwvqsd dooe tcxheds zrdqqhm fdoxv kbxi tlcj
aycnydq qlxhka zoi shplo qll
bfry lbwckm ltq rbfy gpn vojp ruj dpxcve geq
svtvfwh lca lac qia vhwsftv nookdfz xgjiaf yvcdlt
aspgqym fryuzhx bbydf tbn bwutsc fqgi zij lmxhog qnmse
rbb gsys volnas onvlas lonasv vwjdso lnteapy
got iauk kficn jvfuy yvoe jcxwui hyamqx mke mwh jcxwui hyamqx
avutfi ggmha dkopc kothnnb syoi xsd wjedywy
oziejyz yzeijoz hnthyn knj juuq qujtp kgq bymlnlf yicf
zsejuy dybeap hvowmvn okxb yoi epadby cnzjk xfwprzc
lacg iiix fblhxvf nrkkol lnafzw qspzsn gvdy ipj zub uouseo
evukwkh ycjxxc lptwmf pmd izxdsos zrkavf pgjoy zwokg mpjiej
vqw ijwoy eaw wvq svmcq ccxi nyub ynlq eqornax uprt pygfe
plue okbbm btvm gba kutn jacjx ysqt lvx pcxxu qcf
pyw ffjfudq bvk hsdwdva fjnivhf odbmw krpgrj
hziesm bxa dceiwt tmvivjk snl fkh dahsxyx kqlhak lurtk
xss sswyxrg yqff dbkx kbxd mpzbmnl bzplnmm
uvz pjm ilrol pmj uzct ztcu brhkv
heiz jcn syjt zfvlvaq aflvqvz amcjh rxnitw
cxl nxvrn vjnz aewtr cxtko nnvcp ltptd adpxt zvjn fntklj
aymmm tuirj hzngq zhbh paqs kvpfo aqsp kmo acprw sabrso kdqmp
ndqjspv mmhp pndjsvq rti usm
ije oad mvelyg jadz ekm dao zdcmv
qwww tmwmdbb oxxfoza rgmf eonku brh gcgiuoi ojscn
fjedeek ohlax fiydku rbnxpg wfivg cdgs
axwbni hojye mwfe oyqknxp whdgfy ihku mbhr gagnz hehagxj
hibautd blnayq lnayqb gepml mgpel qunw
ircx oeb kujtip zbu ebo cmmn
upyqvot wbponp hnn vav avv tvrky omm
yzqsnf agbfsw dbxoya sfnqzy hqrxek qsnyzf oagyerm xxhukm
xzvk mvcwz oujr hell hoe xexa dqlpqt xdqz ucola hsvv tcmybhl
skldxr mzyol ybzyzd jnnxb rxncdy nkpwy fwlnsw omylz oiwieu fshv ngvha
jkwqf yxrox hejfoq orxyx
rijken xiwf mawqcfu erinjk jsi yyg mmu mdkfqb
ornjes krp eornjs enjros pyqp nnwwjl
wzd uqqo kyeli tikdle aykdjog uiz rbpnw mjxezf ihiz rlgyg
cjm ajqgvkz kfgyy dmczlc mjc kxcm zctyqgh ymsk jwhqfd czpqgan
vxkzvco owo qogj uyictoj kfr pyoo ejrru npluynx bvv jhhzu kuciwc
eqk pcsly kelu arzgoe trfo fotr cuaax
lagonw qvcssqz sdoklh uvovi sfrkmd hnvafj ltg wfjj
viwbkm hpwe kzzwrbr axjtlq mznin wwpjg unlwur
nuzorgo qfoz ydisca qxdfutv hzg
nqgge tobtt hjocx ntyqyi rxzkynw wrnxzyk ciscy trjt ottbt
yuii srawx gljxe eteogz kcu jlgxe tjik ktsnp agudqok jwol vfnyv
vgicg dhnrmxz sjhozy hlalx rutwq
nyoyoje kco hoyam hoyam tta iflud amh gdxcsj vqr fvsqcgv
xdmbtph ueen cskerl rxjvpdc
nricn addljzg obq rikez igq bxygkmv qmgojou uheubk qor
snzd ztusvr vrstzu mceddga hgu
vvrbfjg mcdhmsf ldtwl otuna gmjurrx jgrurxm rxmurjg yrioq
iotkvo sftfvn vvoit lllju xvlg rdsb ywmdf mzxigu kzq
sgqw gqsw lqfu wgqs xpiwou jurgucd azq wgaqpm
ijntzi chlnfj yjqatz hjflcn vys ofq oqf oadthe jrfw
mmc motjo vcwmod rpaszfk zgkkua bpja vjb htrk
bpfvvka kmger mnvvfl hakudy yfprdoo mvnlfv rgmek evnwg
mykpu juavkn cecdvi aszbi lxm hmps oaqoif
fshizd fsdzhi lvcq hhpb eavwno auqlwz rpv owcdojx amsmf qgnddd
pohmcn hlcxk qsesxh rncr
fgyrsis ldem avxmnh frpodq oefzn
plfpu qdyojz xdrzrjy kpv abkh fge bbnotvp liikmcu czvwl oyh
ovha muitw pzy edfjoo fhsxuh dliyruc dikcd cqem ywfy
exyry jtzqn tscr qbtxno cikk poqgr tnjzq eofe sxea anlikep kick
zcie purpw dmhhms bcdo prwup uprpw wfejgjd
kwtjc cmixp dodfwj hcgmmat pkeyspo ubnl ajxvj ffkh xvw
nvlgq oduus psufiqg lrwpn dleftn xtllqvf usgz
liarf sczsf sczsf wky qtzq qvve qvve
cit vtjsh jrhkyvi txj urmq hppx
rhblmxn rhblmxn lkgow dylurwc beyk gfcewxj ehpl disoe tjbjy lkgow
nbkrm jvk ffux ars agns bebic jzjfm kmnbr gptvtsa ufxf
hrlvup jaz tafyr qcgq wkd fiz bgsrx jmtcvo qkbvj
eontk djf tiafrng mtwat puainel nyjoh meynxbf eqdw
aspvmbx tgzuszm fpj xkl nzpr fjp vnomk byx sbtov tnu utn
ldyww gwmiddv hwyh gcgsdit gtgdisc suufl xsw dlwyw
sye dgbd wyf ixqzthx dgdb esy
nsdgera fqz xwbdgui ngdgbcd bcn qrdxml cwcmxws tncm mqsodj cqgk
estayas cocmbpv cdcf vygtswo aplwa estayas
ndc ndc wntr sfls sfls
gse svv esmi lcdii lnr kemrk gnk ildic blnqy wvn
mwlpm awkr sxsudub yauwww hnktbog fpnqc nmxoq yoparu tqjpkug nbipft
czwnkk hrodtmx yyzpil ooqjb cvxzfh
kwa wak gipak gsgrw
jyy fja jjk kuvoqdy urqx
doyu chgn gvtxi qjdigvy kxr dizwrjc sll zenl yyblj
epxeqih kfi hlog pakk kkiidrh hiufw wuhif baqzxzi bgcd phi jzjdxjp
hllhyad sodc nyrtfe kygof hyyqi txddqg wcwxvnt ewqmj wwv
vxymuoe caat diqwbo vfruxdf sqniefn hetcbl nvtttu ouesb
yvoez pvthzc tdowuci wjijicn fhpmq kfobag yctdwj
xaugkb rprkg tidpx pjk tpwwm pbcfhr wmwpt sfynrl iouaw zbnyu
auakc culuxg bffg rodyhea ixlmtfb jdurl szoa
xgona fjzho buh khbvti ddh mgj ptgaqps
dqldupd udpldqd poku gfgpcg zsvk grvk kntx jih uwvxdvq sivk
mwdnq wmqdn uzto mdqnw
alvfm qxqo thwru xqqo jilnsgs rnonk fwntuby ogbha
gvxlxyf cdpv khvpka kgt gshlaa tenb
mtgvvxh mrjrsd truk rrerzx tujweaz
ozepw gsqkr rtmmc cmrtm
spnthg xhlzuu xwcrxz aqqejhh bpzh
ectdftk rgp mkp vxp pevriz wkgfkaw vfygj peg gep wjn
bksbu ywsszf tsbrps vxicr hfustju ynnlbo
sio urbvf ujezjk vkyc ukjezj bvrfu qwwgqmw uqfekvx bzipxus qfumwh
druru kycweog ycmef rjyy fkgp
rmf ifbip rsztco coju wlr bfbmsug lwr bsufbgm nwmp
jjuxtyd yif rkldsvu binq spepa mfg aszm
ghilaau ncm sgbavz omzeotz azukf bgjw zqzo gjbw pld
gtog iqheik budeu guvljmi
qqlj jqql ttk xcxu
cfq cfq kpagib dxfxufw hhksbjh gpcp
xkeax acnia jjubfc mhot uxlhh gnkj pavta rciondm rkquh xudqian
wqhqzg psqh rnnc uujlgq
hpjpaoa maa rdndl xewqj nmagwx xewqj hxuyvou xziv rdndl fbxmbz hmfwghy
dtwnrca hbfcptw qrmvat sdatx les zwizogq
bodiwzg sgoas fsf wgkrn zgbdowi wfkz
ngcsg grtao wcfxpyl gngcs fxwycpl fkpt
txvngo vxngot tkoap zqjc qzcj oeruix myh ihgdfik qtt
rxeh fcbnoo rxeh lve wvoc pmnxej dlcbrh rztt noibg
zyvq lwxqu oyjv bvidmf wxuql
wzc zcw czw dnhkvrg nzslrf
cfgl uwhxu qnsfmt tgyabes mqnq nkitq hmcvxlt qqmn yzmb uomqp
lwziur hgmdmv zuvipkp vir apr gfaq zeo dunat mqgafzg
prq pqkr xlrw njf ncqni kgpoma cmtklv
jwfuc poz opz fuple
fgleub lcgnifu lkwo kftbc onvwvdx lukpod xgmh rnj
rwqvv ezjmoni llq ekd cdvv kzcci gzsj vuipv fnw
rtnua gbnzg kqtogns iozzwc kjpzz kiiurey yzlvzx cpy xrue
fexcjmw ebwssx ewbcgwd uwolou nfdhic vupiykn jss djoo xftbkgo
idf ipvmez qyevwd wfsjxja dif dig
szpbtsa bssaztp sptzasb qppgz odur cpmn wpmg
pxn zjmq rbnr azwstzm mln upaqyty nxp oge nlm
bfaryqv hag phtvh ypi
epeeog lip zqio wuehlnb bau sbd dsb
xbrrp sej agrqnpa aarpnqg bnwyi jbn
uqmsvd asmuyy czxviw pznnmvc
sddwmek wnaea iwphupk sabo
cingdks ksh mtyip zltgafm dflkcd wbdnqup uokm gmxpyd libz svv akce
qge ewv dkabkmb xcpi nrkmsu mkmb djvamg mhhrwjh
krjt etfhm bxzatw zdkvz ehov seyxbw mkiirs plzoplu sogmwb wodfcle
qwea adibdp emo homrd pjcrhlc eqaw kqsrp rphjlcc
gajzo nwjg qxjra jztcnir ijvjwez avxb afz zyywqz kcszgh elmlkfh
lbz ozia bctf bumoji anhil rta xvit
ejybire ypjl qevak fzalx mlh qxlei zib
xmzas kwojjz ntrnrw nbmxlv mdgxs xjhxg suo zdcrxl qkujisz pxmu
eezyd unrtm wyu vhufvto rpb isfcy ygh hgy
nszvbzv ebtt memrsva ebtt qwcaq bhbas pvzfbov ppjbdy nszvbzv jabvrp
rlo zbmi lugvu yeby
tfcd tvl faaq mnural nyarh xnxk ctdf bodz
vwdrhc gub bgu fpcovx rcvwhd jukwsue
aekrhi lpknnrh bett tkib ioqrap igwnst aekrhi lhha
acg mknhazp pcgjuk tajplv
masq fyjkn agq qhxbbl qga npzj fme xtihic rntisg iqv aqg
ipagh fjth mswztpi iexd cocojy vhqrla joe wrsrmw
njztu tsh auqrxca zpp
jctn webxi haq irrr qox irrr webxi
reaw axmnvd voakf lnz ftbxfh zjyxzl pryfjpv sistgb pov mshs
gsy ctsngl ptmnyx vpjx zpvtori pfu ioycdrq
aobdtlj osdnrth sgqe geqs qegs
oamrlxk ygbb rkamoxl nztl sarbmtj yqupjt plu sbtarmj vpa rxea
yvhgp yznko epwpza gqrsod rilukp cglhomj wnaplu ugvdko qdr
cggztg ajw gggzct ubmiefj kpa
rel lvasbh kobm mdnzla pwnyj ehep gzx nhjdnsg rxa
qaz gook rplqwh vsht
dhe aneq ivrn awekad ckcbt zsqca ehd rvni oulwfuu
oxgzzow wntz tkqaoi oxgzzow lwkdpgy lhd aekjasp tkqaoi dnhaw
alxghco cpanoa onjh hyeyebe whxn zfu zozbll gojn
zdqulsa dlqsazu zqudals sfedw
rydtrsv rrtvysd fvyza drdgh lsfzt blnxr cnxe tslzf iijyds ylcxn
cczea nxx kwol kopaza wuvr cyvoo whlicv
zbmrwdq tlzbevx jwzpsc uvkwpd bmss rbzblj
jogx jgi gji hypmtkg ijg oscjv
flkoqja kwmrqv wzehel fvmcfap mkwqvr ivwxg jqfwdvo hweezl
vgjg nzucho nuohcz ggvj tmxci
fqaqx zeybhtg bxeic lftuqp wzuerz sww qfltxk
keiy myrvp blkxcg lncqmsu diittlg fqrf digrel cpwrk ipan dkxb bymlzo
owm irygdz pyhj mow wmo
noul pbvvt zcv ueqyjl zhetlw lpjfhli
felvwb wdykz kyibdz haq qkouj vuav oztyqh
dyxo njcr hcuk ysrr pucw qbajztc
ooyaz pmt hqwu gjx tmp tpm pwz
lyhzajz dfot avyifo kdwka pwypcep kyyw tirlku zdpjmft
aexle hfxo dacwvcy xsiotyg cifq ibupshj aktt rzvf pgafj
pxubhw ibpm jxtxg iwnssf osbpj
exmtfyx blbfg emrunru zkuhoi lfzn zrj unmcece phuppi
icomb rmy mvsqqkh zwjubz lumq wekx
cmdgs gsr pfhqx pfhqx cmdgs pga
rpyf jejc adaiou dutv imbenyu dqw zhebjhu pryf vtxs yprf
cxj roprjn rqoh qacagru snxd
rczvi hfpl luc yowgj nvavlhw vjudkmv dwu teq
klwc cktzh ksnvswl nsgeu xyohp mhs fxnjhm fwrcg rdeadkx cim
ounvb vzqje ujctzzk iyy vxck ebtvbqr uswsmcr jveqz qejzv jmi pboq
lwffygh mqsh vnnj ufz qhms gqfuxo lurzmu
buf psdluck gapwoo wgll sbfavbc lljfvzx cdgo rpt sfvabcb
svefr kubbri fervs nboi zkvq
jwr vtc zkcpzb kczbzp cdned pzbzkc wigjuak fszgweu odflfek
vwdqm khnnj plokjg vnce venc vecn yzxtgb
tawl yrhoz tawl yrhoz
vvehsl kdhzgme rix rcs btm pxnlsps vlhesv sxpnslp yqjtool
eqpyw kpmkcyw wqhglxg ajfzo hbd qvmhy nhokah iisqvad kxuyd fxek
jsz txhwhah hxt djnvl srylveu pxp dzmmn epek tzs
joyzql jqczueb rtdyw fyc fjirfyn tjcalz joyzql fyc
pjrmiz xwnmwns kcqjuut zfgxhdr octwn kqppg zhfgxrd wmwnnxs
ema yqxqs aljjo ajloj wozb
urgmhiz epqj vhhaxdm ptlsvig qzbmm cumbho lkg gyzmg eaopyzf ncfy mqe
ijvwvo oszkees ugvyk hjdj ftip itfp
ylfw qutzdj mgqp cyjss yzsdqqi iykvs fyor sthyqp mrjtzee hgo zwqbtgk
bkfkns gco bykzc mje dwmkrwt ljegqor yxjxp oaleuu
xeltq ggyqis aud frtyxhx iwz wiz fwoxz fozxw
zdu nwduqsa nced iphaaxo
bqjj oah ezd brhgxrc pmkz kdog exw
ihatt hck iepn egemprp wrz wzcuo xjzeaa wku ivjvihh
cwkuof bmj qmxd qbtms zgdei bsqmt ssndhw eeenku lcsqy bvvodr
tek zsgytci vgoun kwwu
jcxvp ijxc buqgix uil zfoku
ggndshq bmjeo yqaxtik blspz yofh edaroy
ipvtxh ouye elln dllvx iqza nhwf zyfw pvlky
iydcx gvarm gvarm wegmiy
sfjd liiflle mulboe qywzs tzbns trojl pad mnfcrhb sltb
gthqj jvpsof jwlfyeg jwhlfj
qckv umzrge gnzc mnr xde
gvgxmhv txnait taxint ius iboqdj
vsfex kbpvsby qembkb efxvs vhflzvm eaazg dyg bbmekq
wxpfk xwfpk xwkpf cjsyi
knzg eefq feqe seppop ttxz qnqfn atgsy cch mkjlbwt uyhct
quzw jbiw miqehe qvf jyipqh kzcjxyh
teuvzf tdtwoi pcuafa cwgjk ccur lgmqv jpjdkk efrnw uloqn dpkjkj lwloeph
yaffjy xntstsv gygq sxttvsn tvnstxs
cvbmdf pfrfkna wupv van iocb hsiyke obspj ytyfkl hbsqtij hkcw
oeddmnu koso mdodeun ybe mhjbmwy ubejz soko yxvuv
nylhy ylnyh olb vcdik
gsp ilba llnu jjk urbvuma qzypf bkceotg ezxq hyvjngf
tfnegyq rue waeif tfnegyq mvqm
wvgnsk cpd oib wrdfaz kohwgkc kzzig hogkwck gkizz
fecuuyp yfq bvanvxb cjeqwf unw dccr qzh zqu voakj
utoazh bjuq kmhcre izmny mirorsy twnl jyoc
fnnpd dmr ccgu eqgewc zuqivf
kkxiba qdabuen oikaz dnuywmm
aogud adugo uzcglpj lucv dgoua mdsqa mvrg
lymhv sof hvyml mlvhy nit
chu bwxp xpbw ghaix seklnc ola zofnrwt uch
wtt abob vblijtd oabb qjws
uozrpw kgf gxidxm uehdr fta pqakkrq atf fat woaolk
gaee voshd ghlyy emvzlkg cmcgk tuwlsj jwtsul znrta mjieqph glker
qiugxas gkg cbzmoz kahs obzzcm
puz omcokz gjc heuqb
dgndhb wid wdi scwnrjf juaisgo eivaw hgdndb
mgcrd hnqg pkpeb vprxcp
atlcnzp fyp cpkivxi bzj ypf cqpt bysu
pnd jiitmzs csw mxnpck vxutdrs ivipzy cws xiegsy qut
txlk avcvbuu hnq yyriq ajyswd urgiwc
qgiqut gvblizs giqnfrk tty mvoj wpikl giqnfrk bkdpndu xztmxn hsmqxf
llthg zjslki wilj rcyfois bavz hrqxn
ytbw hlkl vip skycogy ejiirhx
ndmtg bthlbw lsoq cvlvo sqol sqlo bppl sdkbls dtpyzrq vgm
psm xpj xjp lqi spm gqirw aglpj
htg fcchvyt xffev szdu lieadft
nbjo qohgzu vofg vvild dbtyi pdolxn plnoao jxze xlpbxj brajzg
urpp jjv lihmvp ivkwdqr sesyp ypbry qok sesyp ivkwdqr was
yinepzv qvnzdtf apv ucxo bdioo juga hjfsyl hmowo avc
dmiv tplae iiuiaxx tpale pyzkc
giwhst mpexd byfyc swuzkc
yydkwp xuu vjya kav ujmcxy qrtp zvlk
lsvdyn tkw qxu omvlc wwmfvov mrgcoov dhpu tfair hupd zbx njzgwtw
zuz rsxc xsrc gdwwf nycsv zzu kcu
unlvzv jerqqgm nozma ykbflj qihqkx
pctffo begf ivrvy ezru mvqt waocq
tubtuk gxkc ikgw bjrird kxjebbh sbjyc yafkd khqajmt aclpmf gqfo yrpf
rdt vrxa fyudo myeosb ursflwk
wbjras edlbwdp ctobtw jbvtvcd xjgoo cmunxm mjtbpi klovx bypmsab unc
xckml uztr htublq vilabvr jdiwus qejxur evfw qqm
tzqq tzqq wkb wkb
dgmg ljzc dgmg mbmco cgze qsap jccvot uors iiq
rwvac woylk dmn teorprx nyuvz hcwwxlj lvej drbjo asjgq
ljen tpfl vixcivr guaf lnje waim jlen
djgaa janhi adudm yzv zkcb xqw fgvrz
kpkjoon ggzx skp rqcsw xgzg zgxg jtf ghc
rtnyxo qixfd nphekk mouzk gny fpzquw qgywx rpr gqydze
gawdlv vrivoof rte iyp gaih sfzplm
csojx wzojode uzy qulr lylmb guvtkwv
ovxj aamms ftxo ebckdqw wqvsdci jwfqxks jafrcrn yyomrot
qnu jqwr ywudxk qpsez rdc kiyfz iiecf dthxjjb bown
typ zxcvjo rip acjhl paaab qhqipg xkguye sbxy pomkvn
ofvaegv hgak oafevgv hkemar rqkha grklnsp msvkkku rekahm bxmjnw
ahoihju sdyn phi uhz lupbx
lavt jef klmq oqyfpf kis nazul ymezxek xpla fxyrfnt
nwnagwy hvpjqfg sgm ungfstr gso owqqxjh
hey hye ipyrt qxmthg jth wpbr hxgmtq cvfkfux qykdzhk movcfnl vxyoc
zsras abnrj fgaczuk ssazr xzf cnxu gns wnqqy dwjh szars
uhb zanlvh lvdotkb xekl kcofo
lhx iccy ibkjw ciykxaj imsx ehamqlz iwzapxc rhaltv
pofit owmpqej vwrobh jvox gdqehss yyxd styu tfkm fiotp
ecz mdpoqsv mdpoqsv yxx rexok hcfll yvury hdhcfu juhkvpt rspnfj hxvgdir
ohed mtigaoe eodh agmiteo
vjvv hfco cppbxtw hawsjxz ovlsq qgs risgwhg auhj
togivgg czrtvw ccz wzvtrc bse lsk
ndc ndc lrfi iyleol nchx jxpv xdcsfmp nnx wtvq pih tgc
hzpf sur zhfp klfmhx lbuidp xiqimnf
qddpdk trfxpip pnsowj hidgvnf prur rsrautp aamykfm fysqjmq xwzjane mbmtxhf oqctt
lfd eops govslp ultbye vrqai hcjkcf snpape
cbok koumkad otpozb pqcs emilpe wpcyvxd bock
spjb xkkak anuvk ejoklh nyerw bsjp zxuq vcwitnd xxtjmjg zfgq xkpf
juo pmiyoh xxk myphio ogfyf dovlmwm moevao qqxidn
";

    println!("    * Day 4 - Part 1 ==> {}", day_4_1_2017(input));

    println!("    * Day 4 - Part 2 ==> {}", day_4_2_2017(input));

    let input = "2
0
0
1
2
0
1
-4
-5
0
-1
0
-6
0
-5
2
-9
-11
-15
-3
-11
-12
-14
-5
-16
-3
-13
-6
0
-3
-17
0
-17
-5
-1
-26
-21
-14
-20
-7
-24
-26
-32
-41
-2
-18
-18
-13
-28
0
-32
-3
-2
-14
-17
-54
-22
-34
-33
-34
0
-46
-3
-44
-58
-10
-56
-65
-46
-24
-20
-4
-27
-41
-33
-31
-20
-75
-73
-41
-36
-31
-70
-46
-1
-79
-61
-51
-77
-44
-55
-2
-18
-26
-50
-79
-59
-69
-62
-80
-13
-69
-97
-71
-24
-7
-40
-10
-23
-85
-97
-103
-55
-90
-40
-60
-98
-95
-39
-76
-63
-12
-2
-65
-109
-47
-12
-37
-5
-23
-125
-124
-49
-91
-70
-134
-54
-122
-135
-12
-23
-22
-83
-40
-133
-77
-88
-119
-146
-26
-12
-108
-63
-111
-148
-99
-77
-77
-76
-89
-37
-95
-105
-76
-137
-151
-146
-141
-162
-12
-68
-36
-116
-60
-73
-61
-60
-101
-168
-142
-143
-118
-165
-108
-179
-180
-11
-152
-67
-33
-10
-169
-155
-16
-136
-165
-164
2
1
-28
-131
-86
-153
-116
-113
-149
-66
-64
-36
-168
-116
-159
-15
-180
-125
-146
-135
-105
-161
-133
-207
-192
-192
-99
-146
-93
-21
-5
-189
-86
-16
-4
-44
-167
-20
-201
-110
-103
-223
-182
-71
-194
-68
-90
-237
-147
2
-88
-184
-90
-12
-119
-85
-138
-179
-152
-158
-82
-122
-179
-191
-120
-174
-165
-137
-181
-58
-250
-66
-194
-202
-171
-179
-137
-250
-248
-167
-108
-27
-175
-34
-254
-35
-157
-158
-31
-52
-236
-238
-247
-279
-209
-257
-167
-151
-7
-182
-2
-149
-87
-245
-141
-238
-186
-71
-97
-128
-147
-52
-93
-142
-197
-296
-73
-89
-14
-253
-190
-295
-312
-47
-236
-233
-238
-305
-121
-191
-251
-91
-307
-77
-228
-300
-197
-91
-191
-299
-77
-255
-51
-232
-64
-198
-187
-96
-86
-203
-216
-203
-343
-203
-78
-99
-174
-269
-349
-173
-52
-233
-154
-151
-304
-70
-235
-106
-226
-325
-142
-192
-115
-170
-15
-35
-174
-267
-108
-374
-128
-60
-131
-364
-371
-56
-96
-365
-305
-140
-50
-220
-179
-43
-356
-120
-216
-276
-103
-389
-28
-393
-341
-74
-85
-361
-68
-111
-4
-216
-263
-115
-194
-382
-285
-176
-145
-24
-59
-291
-170
-358
-226
-355
-292
-185
-297
-156
-248
-332
-319
-311
-46
-170
-428
-222
-35
-136
-206
-81
-330
-89
-75
-248
-42
-52
-24
-39
-129
-228
-242
-396
-222
-433
-168
-362
-4
-345
-395
-435
-14
-439
-136
-267
-417
-107
-177
-8
-208
-219
-222
-453
-155
-183
-252
0
-173
-71
-164
-187
-80
-292
-246
-89
-217
-227
-93
-244
-82
-51
-23
-283
-261
-50
-384
-415
-149
-103
-481
-404
-267
-80
-61
-130
-228
-310
-319
-186
-88
-173
-40
-69
-231
-398
-342
-176
-33
-304
-232
-220
-381
-436
-74
-116
-398
-467
-341
-483
-137
-5
-437
-67
-296
-137
-166
-216
-192
-307
-68
-319
-296
-524
-308
-68
-21
-515
-531
-221
-173
-261
-200
-450
-95
-366
-14
-29
-23
-173
-397
-373
-283
-104
-246
-153
-240
-378
-306
-495
-518
-459
-459
-340
-475
-96
-347
-8
-365
-7
-482
-113
-223
-313
-456
-89
-205
-507
-538
-115
-310
-484
-96
-367
-582
-32
-550
-247
-257
-479
-165
-346
-514
-188
-180
-506
-117
-92
-128
-507
-387
-52
-535
-210
-221
-560
-245
-70
-552
-99
-529
-607
-263
-345
-253
-426
-351
-92
-489
-478
-226
-606
-287
-277
-432
-336
-418
-94
-2
-192
-600
-454
-26
-3
-630
-294
-105
-439
-589
-425
-623
-451
-487
-117
-538
-78
-126
-485
-326
-455
-370
-389
-379
-158
-441
-524
-435
-160
-198
-172
-313
-380
-128
-166
-562
-427
-23
-616
-95
-188
-417
-419
-589
-488
-377
-520
-412
-348
-359
-488
-108
-409
-56
-460
-364
-233
-352
-59
-313
-609
-534
-432
-104
-514
-68
-83
-305
-308
-645
-535
-624
-453
-630
-274
-98
-280
-38
-443
-620
-411
-624
-379
-373
-338
-410
-382
-171
-645
-430
-294
-696
-513
-659
-690
-558
-2
-325
-234
-437
-610
-158
-186
-539
-405
-78
-100
-311
-201
-558
-604
-386
-457
-125
-419
-680
-147
-237
-107
-155
-550
-565
-214
-528
-353
-637
-6
-634
-332
-92
-474
-289
-617
-141
-398
-367
-537
-369
-88
-608
-699
-257
-602
-276
-406
-92
-746
-398
-387
-234
-331
-225
-480
-667
-264
-299
-673
-265
-142
-512
-573
-508
-551
-471
-270
-328
-648
-625
-779
-232
-393
-749
-84
-240
-59
-220
-55
-224
-350
-130
-23
-239
-105
-2
-762
-702
-163
-94
-350
-11
-176
-43
-654
-136
-348
-215
-67
-599
-757
-636
-367
-61
-209
-623
-342
-111
-93
-14
-613
-362
-837
-734
-468
-803
-548
-699
-744
-429
-243
-633
-382
-780
-668
-498
-664
-539
-781
-525
-697
-715
-126
-276
-504
-175
-592
-688
-92
-548
-298
-33
-532
-674
-57
-531
-488
-310
-90
-757
-496
-132
-733
-701
-61
-797
-215
-319
-700
-295
-572
-41
-140
-176
-479
-560
-164
-338
-794
-132
-453
-709
-445
-802
2
-336
-562
-802
-878
-547
-368
-502
-574
-275
-687
-560
-432
-423
-174
-367
-59
-605
-340
-626
-142
-601
-488
-299
-466
-521
-783
-140
-731
-779
-252
-663
-906
-410
-601
-524
-332
-750
-556
-730
-749
-294
-798
-93
-345
-316
-186
-634
-904
-237
-134
-765
-953
-170
-854
-910
-99
-782
-564
-505
-49
-827
-64
-297
-548
-841
-598
-414
-184
-67
-99
-880
-855
-722
-725
-582
-416
-473
-339
-491
-162
-311
-43
-938
-608
-524
-212
-4
-945
-544
-879
-382
-21
-512
-169
-284
-140
-588
-407
-56
-610
-75
-412
-321
-801
-881
-220
-388
-116
-962
-1007
-862
-20
-409
-116
-943
-558
-1001
-548
-302
-165
-730
-1012
-669
-875
-393
-979";
    println!("    * Day 5 - Part 1 ==> {}", day_5_1_2017(input));

    println!("    * Day 5 - Part 2 ==> {}", day_5_2_2017(input));

    let input = "0	5	10	0	11	14	13	4	11	8	8	7	1	4	12	11";

    println!("    * Day 6 - Part 1 ==> {}", day_6_1_2017(input));

    println!("    * Day 6 - Part 2 ==> {}", day_6_2_2017(input));
}


fn main() {
    run_advent_of_code_2017();
}
