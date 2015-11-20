module Jekyll
    class TocTag < Liquid::Tag
        def initialize(tag_name, text, tokens)
            super
            @text = text
        end

        def render(context)
            out =   "<div class='toc' markdown='1'>" \
                    "# In this article\n" \
                    "{:.no_toc}\n" \
                    "* TOC\n" \
                    "{:toc}\n" \
                    "\n</div>"
    
            return out
        end
    end
end

Liquid::Template.register_tag('toctag', Jekyll::TocTag)
