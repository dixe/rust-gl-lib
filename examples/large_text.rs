use gl_lib::*;
use gl_lib::sdl_gui as gls;
use failure;


fn main() -> Result<(), failure::Error> {


    let width = 800;
    let height = 600;

    let mut window = gls::window::SdlGlWindow::new("Text window", width, height).unwrap();

    window.window_access().set_swap_interval(0);

    window.set_background_color(na::Vector4::new(0.9, 0.9, 0.9, 1.0));

    window.setup_blend();
    let mut state = State {};
    loop {

        unsafe {
            window.gl().Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }


        let time_ms =  1.0 / window.deltatime();


        window.render_text(&format!("Fps = {}", time_ms));
        window.render_text(&TEST_TEXT);


        window.update(&mut state);

    }
}



#[derive(Debug, Clone)]
enum Message {}

struct State {}


impl gls::State<Message> for State {

    fn handle_message(&mut self, _message: &Message, _window_access: &gls::window::WindowComponentAccess) {

    }


    fn view(&self) -> gls::layout::Node<Message> {
        use gls::layout::*;

        let col = Column::new();

        col.into()
    }
}


const TEST_TEXT: &str = r"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aenean faucibus sem id dui vehicula porttitor. Mauris nibh elit, posuere vel mauris sed, elementum laoreet diam. Etiam hendrerit, orci ac porttitor tristique, velit nisi pretium felis, a posuere risus mi at eros. Suspendisse malesuada tristique tortor ut egestas. Mauris auctor, neque sit amet lobortis accumsan, purus sem sagittis enim, quis pulvinar enim libero id libero. Ut vehicula placerat ligula sed euismod. Aenean sollicitudin ornare est molestie fermentum. Duis egestas mauris dapibus, consectetur sem sit amet, ultrices metus. Pellentesque eget faucibus lorem. Ut pharetra elit quis imperdiet varius. Duis velit felis, rhoncus ac tellus vitae, volutpat pulvinar felis.

                                  Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Nulla dapibus id leo non maximus. In varius neque eget justo vestibulum vehicula. Donec vehicula ligula nibh, vel sagittis tortor interdum non. Donec quis ultrices eros. Aliquam porttitor ex non arcu cursus, non accumsan mauris gravida. Sed neque est, maximus sodales placerat at, convallis non nulla. Praesent nisl leo, ornare et blandit a, pellentesque eget purus. Mauris a pharetra sapien. Maecenas hendrerit turpis id erat efficitur, at rutrum quam scelerisque. Donec fringilla odio ut lorem faucibus aliquet. Integer tempus dui eget nunc accumsan, et cursus dolor tempus. Nulla egestas, dolor quis aliquam porttitor, nibh ex dapibus mi, id sodales quam ex ut nisi. Mauris mollis nibh quis turpis mollis euismod.

                                  Pellentesque a blandit tellus. Aliquam erat volutpat. Cras rhoncus convallis ante id eleifend. Nunc nisi mauris, cursus ut vehicula vel, malesuada at ipsum. Nunc at iaculis nibh, vitae gravida tortor. Donec eget tristique libero, in commodo leo. Proin eu mollis sapien. Cras tincidunt velit risus, sit amet tincidunt odio suscipit nec. Nunc eget egestas arcu. Etiam consectetur sodales purus, nec semper magna porttitor a.

                                  In at libero rutrum, accumsan libero nec, malesuada magna. Donec bibendum lobortis dui, vel ultricies est placerat at. Nam egestas, odio eget consequat efficitur, felis nibh laoreet lorem, eu vestibulum magna nibh ac lorem. Pellentesque in cursus nulla. Mauris nec laoreet nisi, vel fermentum sapien. Ut tincidunt condimentum quam in lobortis. Ut eu consectetur tellus. Donec luctus mi eu dictum fringilla. Quisque condimentum tortor nisi, non dictum nunc ornare non. Pellentesque orci eros, tempor at auctor ac, scelerisque id tellus. In fringilla magna vel orci placerat, at blandit turpis suscipit. Nullam bibendum consequat justo sed blandit. Sed bibendum, nisi vitae rhoncus ornare, ex mi tempor velit, vitae mollis est lacus in sem.

                                  Donec imperdiet nibh vitae dolor auctor, nec interdum ipsum faucibus. Suspendisse consectetur volutpat sem, ac commodo ipsum luctus cursus. Morbi tincidunt sapien ac justo rutrum, sit amet ultrices mi mollis. Mauris ante sem, ornare vitae mattis et, faucibus tristique mi. Cras volutpat congue augue, ac ornare mi sagittis eget. Sed et tincidunt arcu. Morbi ut feugiat augue, eget eleifend nibh. Integer posuere eget sem nec gravida. Phasellus vulputate lobortis erat, ac hendrerit nibh mattis eu. Donec suscipit semper dictum. Phasellus mattis justo nec sem scelerisque elementum. In augue ante, finibus ut euismod id, vestibulum ut odio.

                                  Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. In suscipit euismod pharetra. Nunc commodo quis odio quis volutpat. In ut nisi consequat, eleifend ex vitae, rutrum diam. Morbi ac mollis dolor, nec venenatis velit. Aenean sed ultricies justo. Ut vehicula et sapien vel egestas. Fusce eu vulputate massa. Phasellus luctus vitae diam vitae ultrices. In fringilla mi lacus. Nulla fermentum diam eu lacus faucibus pulvinar. Aliquam convallis bibendum gravida. Nam ultricies fringilla est vel laoreet. Vestibulum posuere ante sit amet tortor dictum rhoncus commodo sit amet metus. Phasellus nisi nulla, varius et feugiat sed, blandit efficitur lacus.

                                  Maecenas ac ligula dolor. Cras non semper risus. Etiam ac sagittis risus. Vivamus et sagittis magna. Vivamus tempor dui et magna ultrices, eleifend auctor neque sagittis. Donec eget leo et arcu vehicula scelerisque. Praesent sed felis a lorem feugiat rhoncus. Donec neque felis, congue congue leo ac, efficitur vulputate arcu. Proin bibendum tincidunt facilisis. Nunc faucibus tellus massa, non blandit diam ornare vel. Fusce pellentesque dui vel ex aliquet volutpat. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Pellentesque auctor dolor at urna dapibus, sed pulvinar lorem placerat. Aenean scelerisque at ante in sagittis. Fusce at ex et massa sollicitudin hendrerit non semper leo. Nam nisl tellus, viverra sit amet quam ac, dignissim vehicula ipsum.

                                  Quisque id dictum nisl. Cras tempus tellus eget dui mollis convallis. Proin sit amet tellus sodales elit facilisis porttitor. Nam felis felis, pretium at magna sit amet, pulvinar consectetur augue. Vestibulum sit amet ipsum eu leo mollis porta. Ut sem purus, accumsan eget mattis ut, convallis vitae metus. Nullam pretium dolor nec dui lacinia, in tempor lacus volutpat. Sed blandit, risus ut tincidunt ornare, enim nisi aliquet sem, in volutpat lectus risus sit amet sem.

                                  In ac consequat neque, sit amet sagittis nisl. Proin laoreet quam non semper vehicula. Praesent sed ligula in ligula dapibus efficitur. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam lorem justo, luctus id leo sed, rutrum molestie eros. Sed magna nisl, suscipit nec pharetra id, pulvinar et nisi. Suspendisse vehicula ante id risus vulputate ultrices. Phasellus ipsum urna, imperdiet vitae consequat quis, sollicitudin a erat. Sed sit amet diam ac nulla tristique laoreet. Integer aliquet enim tellus, eu bibendum mauris fermentum at.

                                  Proin ac maximus magna, a fringilla elit. Vivamus at justo vel nunc tristique malesuada sit amet sit amet ipsum. Aliquam finibus feugiat lorem, viverra dictum enim maximus a. Maecenas bibendum quis ligula vel tincidunt. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Vivamus convallis nisl non diam hendrerit elementum. Praesent pharetra accumsan neque, in venenatis nibh. In eu tempor neque, a pharetra arcu. Sed cursus porta elementum. Aliquam elit nunc, maximus vitae ante sed, rutrum consectetur nibh. Vestibulum nec vehicula libero. Sed non rutrum tortor, vel interdum leo. Donec nec eros nisl. Suspendisse nec commodo massa, non interdum metus.

                                  Integer vel arcu sit amet magna congue scelerisque ac nec lacus. Cras a magna et tellus ultrices semper eu sit amet orci. Duis mollis placerat ligula, a mattis mauris mollis vitae. Praesent tempus nunc sit amet diam molestie, quis tempus tellus maximus. Morbi varius mi neque, a pharetra nibh congue et. Mauris suscipit, purus sed ornare imperdiet, velit justo imperdiet quam, eget iaculis lectus neque non erat. Mauris maximus ac nibh nec consequat. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Proin ultrices, libero quis lacinia maximus, dui risus porttitor turpis, vitae ultricies velit elit vitae dui. Nam quis scelerisque arcu. Nam in viverra ante. Pellentesque eget ligula non dolor egestas suscipit. Duis condimentum arcu non augue aliquet, eu maximus ante dignissim. Sed tellus lectus, suscipit a tincidunt sed, cursus sit amet odio. Nullam bibendum elementum tellus, nec scelerisque leo faucibus id.

                                  Nam orci lectus, sagittis at congue sed, tempor nec mauris. Interdum et malesuada fames ac ante ipsum primis in faucibus. Nunc ornare mauris non metus molestie, in suscipit risus vulputate. Maecenas mattis metus lobortis facilisis sollicitudin. Aenean id congue quam. Sed semper tincidunt justo sit amet pretium. Suspendisse scelerisque quam et suscipit maximus. Quisque facilisis velit eu lectus tincidunt, ut euismod felis accumsan. Praesent vitae sapien in eros porttitor consectetur. Maecenas ac magna nec sem aliquam tincidunt vitae sed tortor. In mi diam, lobortis sed fringilla sit amet, molestie in eros. Nunc interdum molestie quam in sodales. Sed nec euismod risus. Aenean interdum congue est nec pharetra. Aenean nec purus sed libero condimentum mattis.

                                  Donec non rutrum tellus. Curabitur malesuada vehicula nibh, a malesuada tellus feugiat eu. Proin sollicitudin arcu id ex auctor eleifend. In rutrum tempor mollis. Nullam viverra arcu et lacus interdum euismod. Nulla facilisi. Nullam vel efficitur nisi, id venenatis magna. Duis nec ante feugiat, aliquet augue vel, volutpat magna. Suspendisse lacinia lorem sit amet tincidunt euismod.

                                  Duis a lacus sit amet arcu sollicitudin cursus. Vivamus facilisis, mi sit amet bibendum imperdiet, ante lectus varius arcu, eget volutpat neque nisl sed metus. Proin eget libero eget massa venenatis aliquet quis nec sapien. Fusce vitae sapien augue. Pellentesque sem lectus, congue id turpis ut, tincidunt eleifend velit. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Cras vehicula lorem nec nulla ultrices tincidunt. Integer id convallis ipsum. Duis consequat accumsan nulla, in consectetur quam ultrices et. Cras in nulla augue.

                                  Nunc vel ipsum nisl. Praesent a mi vel ante pharetra bibendum. Proin vel nunc ut quam hendrerit rutrum eget ut turpis. Nunc imperdiet id lectus quis porttitor. Quisque pulvinar leo pharetra tortor congue, vel lacinia massa mattis. Suspendisse laoreet urna ut semper consectetur. Vivamus augue nisi, aliquam sed felis sed, luctus fermentum tortor. In hac habitasse platea dictumst. Nullam ultricies vestibulum consequat. Aliquam porttitor vulputate elementum. Vestibulum sit amet sollicitudin ante. Suspendisse ac hendrerit urna. Duis consectetur pulvinar nulla, sed semper urna scelerisque sit amet. Duis eu ipsum cursus risus porta ultricies.

                                  Nullam non leo auctor ligula tincidunt efficitur. Aliquam elementum aliquam augue. Aliquam sit amet sollicitudin lacus, non consectetur dolor. Etiam convallis molestie fermentum. Nunc dapibus dui ac facilisis mattis. Donec lobortis pellentesque est ut aliquet. Nunc euismod egestas dolor, ac lacinia quam lobortis ac. Morbi viverra, felis lacinia blandit ornare, eros leo eleifend erat, ut ullamcorper massa magna cursus metus. Aliquam at mi massa. Nam pretium suscipit neque ut hendrerit.

                                  Duis a vestibulum urna, id ultricies ligula. Sed auctor interdum purus molestie ultrices. Maecenas condimentum, leo non pharetra hendrerit, turpis augue pellentesque justo, sit amet suscipit nulla tellus ut magna. Pellentesque in molestie ipsum. Nullam vehicula metus sit amet laoreet gravida. Nam vitae felis laoreet, sodales tellus in, scelerisque tortor. Nulla posuere augue tristique pellentesque commodo. In quis odio vulputate, consectetur libero quis, viverra eros. Nulla facilisis ligula dolor, quis aliquam lacus ultrices at. Suspendisse sit amet finibus justo. Sed at fermentum nunc. Fusce varius, tellus ut rutrum scelerisque, augue eros rutrum neque, non pharetra eros nunc at sapien. Donec luctus diam et orci placerat molestie. Sed eu aliquet orci. Suspendisse finibus nisi at dapibus posuere. Sed rhoncus odio felis, vel efficitur orci porta in.

                                  Aenean maximus id lectus quis condimentum. Fusce mollis libero ipsum, ut malesuada velit elementum id. In eu ligula elit. Sed ullamcorper lacinia porttitor. Nunc eu dignissim diam, in tempus arcu. Integer mattis arcu at purus sodales sollicitudin. Nulla et faucibus urna. Curabitur quis ante ac ex pellentesque venenatis. Donec venenatis pretium blandit.

                                  Morbi vel ipsum volutpat, pretium neque aliquam, commodo nisl. In sem diam, dapibus eget ex et, imperdiet dapibus arcu. Duis eget odio sed ante eleifend mollis quis non nisi. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Proin facilisis justo nec ipsum dapibus, vel venenatis neque hendrerit. Mauris non urna eget augue cursus laoreet. Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. In ut sapien tortor. Praesent porttitor facilisis velit et condimentum. Aenean nisl eros, egestas nec sagittis eget, vulputate eget nunc. Morbi id orci neque.

                                  In suscipit orci felis, eget facilisis lectus rhoncus id. Vestibulum quis vehicula ante, vitae porta orci. Etiam egestas tellus metus. Sed aliquet blandit erat, nec lacinia lectus elementum at. Sed et velit augue. Nullam volutpat tellus vitae justo imperdiet, ut pharetra arcu molestie. Aliquam vitae sapien ligula. Praesent ac arcu vel erat finibus viverra in non libero. Phasellus eget sem vel turpis varius dignissim quis non dolor. Phasellus pellentesque diam et placerat fermentum. Aliquam urna libero, egestas placerat nulla vitae, feugiat vestibulum ex. Morbi magna dui, faucibus ut consectetur eget, ultrices porta eros. Etiam facilisis neque a purus lacinia porta. Praesent fermentum urna ut felis malesuada, in molestie orci sollicitudin.Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec eu hendrerit velit. Vestibulum congue dui id laoreet viverra. Suspendisse ornare, velit in facilisis feugiat, elit orci viverra leo, sit amet consectetur nunc enim non mi. Curabitur sed efficitur lacus. Duis eu viverra nunc. Duis cursus maximus turpis. Aenean a convallis nulla.

                                  Fusce lorem mauris, scelerisque vitae scelerisque quis, posuere viverra mauris. Ut convallis nec ex non tincidunt. Duis sapien est, vulputate vitae mattis eu, egestas vitae dolor. Suspendisse nec orci quis sem pharetra rhoncus. Donec egestas euismod ultrices. Nam euismod sem lorem, ut accumsan turpis congue sit amet. Integer ut tortor sit amet leo sagittis convallis sed quis augue. Quisque non magna placerat, consequat mi ut, hendrerit neque. Nunc sit amet fringilla lacus, non tempus lorem. Sed gravida consectetur nulla, sed maximus neque scelerisque eget. Donec quis risus metus. Duis eget dui id mi consectetur consequat eget in ex. Lorem ipsum dolor sit amet, consectetur adipiscing elit.

                                  Vivamus pulvinar sapien at aliquam egestas. Etiam quis nisl vel velit euismod blandit. Maecenas rhoncus erat gravida mi facilisis, tempor egestas turpis venenatis. Duis ac lacus nec lectus lobortis porta. Aliquam aliquam, ex in feugiat cursus, risus dui maximus felis, eget mattis mi erat et odio. Aliquam tincidunt at urna vel placerat. Vestibulum bibendum, quam eu viverra eleifend, sem elit venenatis est, vel volutpat ante leo id metus. Vivamus luctus ligula sit amet vulputate aliquet. Nam et tincidunt nulla, non ultrices massa. Curabitur venenatis orci vel urna sollicitudin, eu sagittis eros fringilla. Aenean sed libero quis massa tempor imperdiet vitae quis massa. Sed faucibus interdum elit, eget lobortis leo fermentum nec. Nulla facilisi. Fusce ac tincidunt nisl. Donec fringilla leo a vestibulum varius.

                                  Fusce fringilla augue in erat faucibus aliquam. Aliquam erat volutpat. Nam euismod augue sagittis libero rhoncus, auctor aliquam eros rhoncus. Sed a lacus quis lectus volutpat euismod. Donec non consectetur augue. Suspendisse ultricies aliquam turpis, quis convallis justo vehicula et. Maecenas molestie, sem non aliquam tempor, massa sapien ultrices mi, et tristique lorem odio in mauris. In tempus id magna eget vulputate.

                                  Donec molestie in mauris et euismod. In posuere mauris dolor, sed aliquet orci accumsan ut. Curabitur velit purus, rhoncus ut dui sit amet, sollicitudin suscipit ipsum. Cras venenatis felis sed ex blandit convallis. Vivamus vestibulum mollis justo id rhoncus. Pellentesque fringilla maximus aliquet. Praesent luctus odio sed lacus bibendum mattis.
                                  ";
