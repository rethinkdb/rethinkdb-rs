module Jekyll
    class InfoboxBlockTag < Liquid::Block
        def initialize(tag_name, classes, tokens)
            super
            @infobox_classes = classes
        end

        def render(context)
            site = context.registers[:site]
            converter = site.getConverterImpl(Jekyll::Converters::Markdown)
            content = converter.convert(super.strip)
            "<div class='infobox #{@infobox_classes.split(' ').map{ |x| "infobox-#{x}" }.join(' ')}'>#{content}</div>".strip
        end
    end
end

Liquid::Template.register_tag('infobox', Jekyll::InfoboxBlockTag)
